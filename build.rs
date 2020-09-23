use std::env;

fn main() {
    println!("cargo:rustc-link-lib=static=glfw3");
    println!(r"cargo:rustc-link-search=native={}", env::var("GLFW_LIB_DIR").unwrap());
}