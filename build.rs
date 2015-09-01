extern crate gcc;

#[cfg(target_os = "linux")]
fn main() {
    gcc::compile_library("libtiocgwinsz.a", &["native/tiocgwinsz.c"]);
}

#[cfg(target_os = "windows")]
fn main() {}
