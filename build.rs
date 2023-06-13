use std::{env, path::PathBuf, process::Command};

fn main() {
    let rustc = env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    let output = Command::new(rustc)
        .args(&["--print", "sysroot"])
        .output()
        .expect("Could not invoke `rustc` to find rust sysroot");
    
    let path = String::from_utf8(output.stdout)
        .unwrap()
        .trim()
        .to_owned();

    let lib_path = PathBuf::from(&path).join("lib");
    
    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_path.display());
}
