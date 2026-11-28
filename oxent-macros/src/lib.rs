use proc_macro::{Delimiter, TokenStream, TokenTree};

#[proc_macro_derive(Model)]
pub fn derive_model(input: TokenStream) -> TokenStream {
    let out = match parse(input) {
        Ok(Item::Struct(name, fields)) => struct_impl(&name, &fields),
        Ok(Item::Enum(name, variants)) => enum_impl(&name, &variants),
        Err(msg) => format!("::core::compile_error!({msg:?});"),
    };
    out.parse().expect("generated Model impl")
}

enum Item {
    Struct(String, Vec<(String, String)>),
    Enum(String, Vec<(String, Vec<String>)>),
}

fn parse(input: TokenStream) -> Result<Item, String> {
    let mut tokens = input.into_iter().peekable();
    let mut kind = None;
    for t in tokens.by_ref() {
        if let TokenTree::Ident(id) = &t {
            let s = id.to_string();
            if s == "struct" || s == "enum" {
                kind = Some(s);
                break;
            }
        }
    }
    let kind = kind.ok_or("Model can only be derived for a struct or an enum")?;
    let name = match tokens.next() {
        Some(TokenTree::Ident(id)) => id.to_string(),
        _ => return Err("expected a type name".into()),
    };
    let body = match tokens.next() {
        Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace => g.stream(),
        Some(TokenTree::Punct(p)) if p.as_char() == '<' => return Err("Model does not support generic types".into()),
        _ => return Err("Model needs a struct with named fields or an enum".into()),
    };
    if kind == "struct" { Ok(Item::Struct(name, named_fields(body)?)) } else { Ok(Item::Enum(name, variants(body)?)) }
}

fn split_top(stream: TokenStream) -> Vec<Vec<TokenTree>> {
    let mut parts = vec![Vec::new()];
    let mut depth = 0i32;
    let mut prev_dash = false;
    for t in stream {
        if let TokenTree::Punct(p) = &t {
            match p.as_char() {
                '<' => depth += 1,
                '>' if !prev_dash => depth -= 1,
                ',' if depth == 0 => {
                    parts.push(Vec::new());
                    prev_dash = false;
                    continue;
                }
                _ => {}
            }
            prev_dash = p.as_char() == '-';
        } else {
            prev_dash = false;
        }
        parts.last_mut().unwrap().push(t);
    }
    parts.retain(|p| !p.is_empty());
    parts
}

fn strip_prefix(part: &[TokenTree]) -> &[TokenTree] {
    let mut i = 0;
    while i < part.len() {
        match &part[i] {
            TokenTree::Punct(p) if p.as_char() == '#' => i += 2,
            TokenTree::Ident(id) if id.to_string() == "pub" => {
                i += 1;
                if matches!(part.get(i), Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Parenthesis) {
                    i += 1;
                }
            }
            _ => break,
        }
    }
    &part[i..]
}

fn tokens_string(tokens: &[TokenTree]) -> String {
    tokens.iter().cloned().collect::<TokenStream>().to_string()
}

fn named_fields(body: TokenStream) -> Result<Vec<(String, String)>, String> {
    split_top(body)
        .iter()
        .map(|part| {
            let part = strip_prefix(part);
            match (part.first(), part.get(1)) {
                (Some(TokenTree::Ident(id)), Some(TokenTree::Punct(c))) if c.as_char() == ':' => {
                    Ok((id.to_string(), tokens_string(&part[2..])))
                }
                _ => Err("Model needs named fields".into()),
            }
        })
        .collect()
}

fn variants(body: TokenStream) -> Result<Vec<(String, Vec<String>)>, String> {
    split_top(body)
        .iter()
        .map(|part| {
            let part = strip_prefix(part);
            let Some(TokenTree::Ident(id)) = part.first() else { return Err("expected a variant name".into()) };
            let types = match part.get(1) {
                Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Parenthesis => {
                    split_top(g.stream()).iter().map(|t| tokens_string(strip_prefix(t))).collect()
                }
                Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace => {
                    named_fields(g.stream())?.into_iter().map(|(n, t)| format!("{n}:{t}")).collect()
                }
                _ => Vec::new(),
            };
            Ok((id.to_string(), types))
        })
        .collect()
}

const HOT: &str = "::oxent::hot";

fn struct_impl(name: &str, fields: &[(String, String)]) -> String {
    let entries: String = fields
        .iter()
        .map(|(f, ty)| {
            format!(
                "{HOT}::FieldDesc {{ name: {f:?}, offset: ::core::mem::offset_of!({name}, {f}), payload: 0, ty: <{ty} as {HOT}::Layout>::DESC }},"
            )
        })
        .collect();
    format!(
        "#[allow(non_upper_case_globals)]
        const __OXENT_FIELDS_{name}: &[{HOT}::FieldDesc] = &[{entries}];
        unsafe impl {HOT}::Layout for {name} {{
            const DESC: &'static {HOT}::TypeDesc = &{HOT}::TypeDesc::structure::<{name}>({name:?}, __OXENT_FIELDS_{name});
        }}"
    )
}

fn enum_impl(name: &str, variants: &[(String, Vec<String>)]) -> String {
    let unit = variants.iter().all(|(_, t)| t.is_empty());
    let entries: String = variants
        .iter()
        .map(|(v, types)| {
            let prints: String = types
                .iter()
                .map(|t| match t.split_once(':') {
                    Some((n, ty)) if !ty.starts_with(':') => {
                        format!("{HOT}::fnv({HOT}::fnv(0, {n:?}.as_bytes()), &<{ty} as {HOT}::Layout>::DESC.print.to_le_bytes()),")
                    }
                    _ => format!("<{t} as {HOT}::Layout>::DESC.print,"),
                })
                .collect();
            format!("{HOT}::FieldDesc {{ name: {v:?}, offset: 0, payload: {HOT}::combine(&[{prints}]), ty: {HOT}::UNIT }},")
        })
        .collect();
    let table = format!(
        "#[allow(non_upper_case_globals)]
        const __OXENT_FIELDS_{name}: &[{HOT}::FieldDesc] = &[{entries}];"
    );
    if !unit {
        return format!(
            "{table}
            unsafe impl {HOT}::Layout for {name} {{
                const DESC: &'static {HOT}::TypeDesc = &{HOT}::TypeDesc::opaque_enum::<{name}>({name:?}, __OXENT_FIELDS_{name});
            }}"
        );
    }
    let reads: String = variants.iter().enumerate().map(|(i, (v, _))| format!("{name}::{v} => {i},")).collect();
    let writes: String = variants.iter().enumerate().map(|(i, (v, _))| format!("{i} => {name}::{v},")).collect();
    format!(
        "{table}
        unsafe impl {HOT}::Layout for {name} {{
            const DESC: &'static {HOT}::TypeDesc = &{HOT}::TypeDesc::unit_enum::<{name}>({name:?}, __OXENT_FIELDS_{name}, {{
                unsafe fn read(p: *const u8) -> u32 {{ match unsafe {{ &*(p as *const {name}) }} {{ {reads} }} }}
                read
            }}, {{
                unsafe fn write(p: *mut u8, i: u32) {{
                    let v = match i {{ {writes} _ => return }};
                    unsafe {{ ::core::ptr::write(p as *mut {name}, v) }}
                }}
                write
            }});
        }}"
    )
}
