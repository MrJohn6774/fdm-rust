fn main() {
    println!("cargo:rustc-link-search=native={}", "libuipc");
    println!("cargo:rustc-link-lib=static=libuipc");
    println!("cargo:rustc-link-lib=dylib=user32");
}
