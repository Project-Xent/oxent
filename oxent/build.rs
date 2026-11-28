use std::env;
use std::path::PathBuf;
use std::process::Command;

const RUNTIME_SOURCES: &[&str] = &["src/lib.rs", "src/reconcile.rs", "src/ffi.rs", "src/hot.rs"];

fn main() {
    let rustc = env::var("RUSTC").unwrap_or_else(|_| "rustc".into());
    let version = Command::new(rustc)
        .arg("-V")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default();
    println!("cargo:rustc-env=OXENT_RUSTC={version}");

    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for file in RUNTIME_SOURCES {
        println!("cargo:rerun-if-changed={file}");
        for b in std::fs::read(file).unwrap_or_default() {
            hash = (hash ^ b as u64).wrapping_mul(0x0100_0000_01b3);
        }
    }
    println!("cargo:rustc-env=OXENT_RUNTIME={hash:016x}");

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let profile = out
        .ancestors()
        .find(|p| p.ends_with("build"))
        .and_then(|p| p.parent())
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .unwrap_or("release");
    println!("cargo:rustc-env=OXENT_PROFILE={profile}");
}
