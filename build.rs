extern crate rustc_version;

use std::env;
use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    if probe("fn main() { 0i128; }") {
        println!("cargo:rustc-cfg=has_i128");
    } else if env::var_os("CARGO_FEATURE_I128").is_some() {
        panic!("i128 support was not detected!");
    }

    match rustc_version::version_meta() {
        Ok(ref meta) if meta.semver.major >= 1 && meta.semver.minor >= 32 => {
            println!("cargo:rustc-cfg=int_to_from_bytes");
        }
        Ok(ref meta)
            if env::var_os("CARGO_FEATURE_INT_TO_FROM_BYTES").is_some()
                && meta.channel == rustc_version::Channel::Stable =>
        {
            panic!("`int_to_from_bytes` support was not stabilizations!");
        }
        _ => {}
    }
}

/// Test if a code snippet can be compiled
fn probe(code: &str) -> bool {
    let rustc = env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    let out_dir = env::var_os("OUT_DIR").expect("environment variable OUT_DIR");

    let mut child = Command::new(rustc)
        .arg("--out-dir")
        .arg(out_dir)
        .arg("--emit=obj")
        .arg("-")
        .stdin(Stdio::piped())
        .spawn()
        .expect("rustc probe");

    child
        .stdin
        .as_mut()
        .expect("rustc stdin")
        .write_all(code.as_bytes())
        .expect("write rustc stdin");

    child.wait().expect("rustc probe").success()
}
