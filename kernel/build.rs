fn main() {
    println!("cargo:rerun-if-env-changed=LOG");

    match option_env!("ARCH") {
        Some(arch) => { println!("cargo:rustc-cfg={}", arch); }
        None => { println!("cargo:rustc-cfg=riscv64"); }
    }
}