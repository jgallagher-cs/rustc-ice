fn main() {
    println!("cargo:rustc-link-search=foo-mac");
    println!("cargo:rustc-link-lib=static=foo");
}
