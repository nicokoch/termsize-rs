extern crate gcc;

#[cfg(target_os = "linux")]
fn main() {
    gcc::compile_library("libtiocgwinsz.a", &["native/tiocgwinsz.c"]);
}
