use std::env;
use std::path::PathBuf;

const LIBS: &[&str] = &[
    "fluxent",
    "xtk",
    "xent_core",
    "cwinrt-rt",
    "cwinrt-bindings-foundation",
    "cwinrt-bindings-ui-composition",
    "cwinrt-bindings-ui-composition-interactions",
    "cwinrt-bindings-ui-viewmanagement",
    "cwinrt-bindings-globalization",
    "cwinrt-bindings-globalization-datetimeformatting",
    "cwinrt-bindings-globalization-numberformatting",
    "cwinrt-bindings-ui-input",
];

const SYSLINKS: &[&str] = &[
    "user32",
    "gdi32",
    "dcomp",
    "d2d1",
    "d3d11",
    "dxgi",
    "dwrite",
    "dwmapi",
    "ole32",
    "oleaut32",
    "uuid",
    "uxtheme",
    "imm32",
    "advapi32",
    "shell32",
    "coremessaging",
    "uiautomationcore",
    "runtimeobject",
    "windowscodecs",
];

fn arch() -> &'static str {
    match env::var("CARGO_CFG_TARGET_ARCH").as_deref() {
        Ok("aarch64") => "aarch64",
        _ => "x86_64",
    }
}

fn lib_dir() -> PathBuf {
    if let Ok(dir) = env::var("XENT_LIB_DIR") {
        return PathBuf::from(dir);
    }
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let profile = if env::var("PROFILE").as_deref() == Ok("release") { "release" } else { "debug" };
    let tree = if arch() == "aarch64" { "arm64" } else { "x86_64" };
    let local = manifest.join("../../fluxent/build/mingw").join(tree).join(profile);
    if local.join("libfluxent.a").exists() { local } else { manifest.join("native").join(arch()) }
}

fn copy_runtime(dir: &std::path::Path) {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let Some(profile) = out.ancestors().find(|p| p.ends_with("build")).and_then(|p| p.parent()) else { return };
    for dest in [profile.to_path_buf(), profile.join("examples")] {
        let _ = std::fs::create_dir_all(&dest);
        let _ = std::fs::copy(dir.join("libxent_dev.dll"), dest.join("libxent_dev.dll"));
    }
}

fn main() {
    println!("cargo:rerun-if-env-changed=XENT_LIB_DIR");
    if env::var_os("DOCS_RS").is_some() {
        return;
    }
    let dir = lib_dir();
    println!("cargo:rustc-link-search=native={}", dir.display());
    if env::var_os("CARGO_FEATURE_HOT").is_some() {
        let dll = dir.join("libxent_dev.dll.a");
        assert!(dll.exists(), "missing {}; build fluxent's xent_dev target first", dll.display());
        println!("cargo:rerun-if-changed={}", dll.display());
        println!("cargo:rustc-link-lib=dylib=xent_dev");
        copy_runtime(&dir);
        for lib in SYSLINKS {
            println!("cargo:rustc-link-lib=dylib={lib}");
        }
        return;
    }
    let bundle = dir.join("libxent.a").exists();
    for lib in if bundle { &["xent"][..] } else { LIBS } {
        let file = dir.join(format!("lib{lib}.a"));
        assert!(file.exists(), "missing {}", file.display());
        println!("cargo:rerun-if-changed={}", file.display());
        println!("cargo:rustc-link-lib=static={lib}");
    }
    for lib in SYSLINKS {
        println!("cargo:rustc-link-lib=dylib={lib}");
    }
}
