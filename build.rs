fn main() {
    println!("cargo:rustc-link-search=native={}","libfsuipc");
    println!("cargo:rustc-link-lib=static=libfsuipc");
    println!("cargo:rustc-link-lib=dylib=user32");
}
