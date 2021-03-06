extern crate termsize;
extern crate regex;

use termsize::termsize;
use std::env;

#[test]
fn size_not_zero() { // TODO being zero is actually valid, why are we even doing this.
    if let Ok(_) = env::var("APPVEYOR") {return}
    let (width, height) = termsize().unwrap();
    assert!(width > 0);
    assert!(height > 0);
    println!("Terminal size: {:?}", (width, height));
}

#[test]
fn works_in_child_thread() { //TODO also implement for child process
    use std::thread;
    if let Ok(_) = env::var("APPVEYOR") {return}
    let (width, height) = termsize().unwrap();
    let handle = thread::spawn(|| {
        termsize().unwrap()
    });
    let child_dimensions = handle.join().unwrap();
    assert_eq!(child_dimensions.0, width);
    assert_eq!(child_dimensions.1, height);
}

#[cfg(target_os = "linux")]
#[test]
fn matches_resize_command() {
    use std::process::{Command, Output};
    use regex::Regex;

    let (width_stty, height_stty): (usize, usize) = match Command::new("resize").output() {
        Ok(Output { stderr: _, stdout: out, status: _ }) => {
            let output = String::from_utf8(out).unwrap();
            let output_lines = output.lines().collect::<Vec<&str>>();
            // Columns
            let re = Regex::new(r"^COLUMNS=([0-9]+);$").unwrap();
            let captures = re.captures(output_lines[0]).unwrap();
            let width = captures.at(1).unwrap();
            println!("{}", width);

            //Rows
            let re = Regex::new(r"^LINES=([0-9]+);$").unwrap();
            let captures = re.captures(output_lines[1]).unwrap();
            let height = captures.at(1).unwrap();
            println!("{}", height);

            (width.parse().unwrap(), height.parse().unwrap())
        }
        Err(_) => return
    };

    let (width, height) = termsize().unwrap();
    assert_eq!(width, width_stty);
    assert_eq!(height, height_stty);
}
