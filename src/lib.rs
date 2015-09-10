#![warn(missing_docs)]
//! This crate provides a single function, `termsize()`,
//! which returns the size of the terminal in columns and rows.
//! # Usage
//! ```rust
//! extern crate termsize;
//! use termsize::termsize;
//!
//! fn main() {
//!     match termsize() {
//!         Some((columns, rows)) => println!("Size of terminal is {} x {}", columns, rows),
//!         None => println!("Not called from a terminal")
//!     }
//! }
//! ```

/// Returns the size of the terminal as a tuple `Option<(width, height)>`.
///
/// Width: Number of columns.
///
/// Height: Number of rows.
pub fn termsize() -> Option<(usize, usize)> {
    target::termsize()
}

#[cfg(target_os = "linux")]
mod target {
    extern crate libc;

    use std::mem;
    use self::libc::{c_int, c_ushort, STDOUT_FILENO};
    use self::libc::funcs::bsd44::ioctl;

    extern {
        static tiocgwinsz: c_int;
    }

    #[repr(C)]
    struct WinSize {
        ws_row: c_ushort,
        ws_col: c_ushort,
        ws_xpixel: c_ushort, /* unused */
        ws_ypixel: c_ushort, /* unused */
    }

    pub fn termsize() -> Option<(usize, usize)> {
        unsafe {
            let mut ws: WinSize = mem::zeroed();
            if ioctl(STDOUT_FILENO, tiocgwinsz, &mut ws as *mut _) == 0 {
                Some((ws.ws_col as usize, ws.ws_row as usize))
            } else {
                None
            }
        }
    }
}

#[cfg(target_os = "windows")]
mod target {
    extern crate winapi;
    extern crate kernel32;

    use std::mem;
    use self::winapi::{wincon, winbase};

    // TODO: this method works for cmd and powershell,
    // but returns (0, 0), if you use i.e. bash shell on windows.
    // Possible fix: implement a fallback method which utilizes tput, resize or stty.
    pub fn termsize() -> Option<(usize, usize)>{
        let csbi = unsafe {
            let mut csbi: wincon::CONSOLE_SCREEN_BUFFER_INFO = mem::zeroed();
            let std_handle = kernel32::GetStdHandle(winbase::STD_OUTPUT_HANDLE);
            if kernel32::GetConsoleScreenBufferInfo(std_handle, &mut csbi as wincon::PCONSOLE_SCREEN_BUFFER_INFO) == 0 {
                return None
            }
            csbi
        };

        let columns = csbi.srWindow.Right - csbi.srWindow.Left + 1;
        let rows = csbi.srWindow.Bottom - csbi.srWindow.Top + 1;
        Some((columns as usize, rows as usize))
    }
}
