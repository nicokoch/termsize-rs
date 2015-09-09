# termsize-rs
[![Build Status](https://travis-ci.org/nicokoch/termsize-rs.svg?branch=master)](https://travis-ci.org/nicokoch/termsize-rs)
[![Build status](https://ci.appveyor.com/api/projects/status/2vx0f7p7intuxafn/branch/master?svg=true)](https://ci.appveyor.com/project/nicokoch/termsize-rs/branch/master)

[Documentation](http://nicokoch.github.io/termsize-rs/termsize/)

Get the terminal size in columns and rows.

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
