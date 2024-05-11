fn main() {
    println!("cargo:rerun-if-env-changed=LOG");

    match option_env!("ARCH") {
        Some(arch) => { println!("cargo:rustc-check-cfg=cfg({})", arch); }
        None => { println!("cargo:rustc-check-cfg=cfg(riscv64)"); }
    }
}