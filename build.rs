fn main() {
    println!("cargo:rustc-link-search=native={}","libfsuipc");
    println!("cargo:rustc-link-lib=static=libuipc");
    println!("cargo:rustc-link-lib=dylib=user32");
}
