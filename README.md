# termsize-rs
Get the terminal size in columns and rows

This crate provides a single function, `termsize()`,
which returns the size of the terminal in columns and rows.
## Usage
```rust
extern crate termsize;
use termsize::termsize;

fn main() {
     match termsize() {
         Some((columns, rows)) => println!("Size of terminal is {} x {}", columns, rows),
         None => println!("Not called from a terminal")
     }
}
```
