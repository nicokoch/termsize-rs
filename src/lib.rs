pub fn termsize() -> Option<(usize, usize)> {
    target::termsize()
}

#[cfg(target_os = "linux")]
mod target {
    extern crate libc;
    use self::libc::{c_int, c_ushort, STDOUT_FILENO};
    use self::libc::funcs::bsd44::ioctl;

    extern {
        static tiocgwinsz: c_int;
    }

    #[repr(C)]
    #[derive(Default)]
    struct WinSize {
        ws_row: c_ushort,
        ws_col: c_ushort,
        ws_xpixel: c_ushort, /* unused */
        ws_ypixel: c_ushort, /* unused */
    }

    pub fn termsize() -> Option<(usize, usize)> {
        let ws = &mut WinSize::default();
        unsafe {
            if ioctl(STDOUT_FILENO, tiocgwinsz, ws as *mut WinSize) == 0 {
                Some((ws.ws_col as usize, ws.ws_row as usize))
            } else {
                None
            }
        }
    }
}
