fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = std::path::Path::new(&out_dir);
    let asm_dir = std::env::current_dir().unwrap().join("asm");

    let nasm_result = std::process::Command::new("nasm")
        .args(&["-f", "elf64", "-o"])
        .arg(out_dir.join("factorial.o"))
        .arg(asm_dir.join("factorial.asm"))
        .status()
        .expect("Failed to compile factorial.asm");
    assert!(nasm_result.success());

    let ar_result = std::process::Command::new("ar")
        .args(&[
            "crus",
            out_dir.join("libfactorial.a").to_str().unwrap(),
            out_dir.join("factorial.o").to_str().unwrap(),
        ])
        .current_dir(out_dir)
        .status()
        .expect("Failed to create libfactorial.a");
    assert!(ar_result.success());

    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rustc-link-lib=factorial");
    println!(
        "cargo:rerun-if-changed={}",
        asm_dir.join("factorial.asm").display()
    );
}
