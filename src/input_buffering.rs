use std::mem::zeroed;
use std::os::unix::io::RawFd;

use libc::{c_int, fd_set, select, timeval, FD_SET, FD_ZERO, STDIN_FILENO};
use termios::*;

use crate::utils::flush_stdout;

pub fn disable_input_buffering() -> Termios {
    let mut tio = Termios::from_fd(STDIN_FILENO).unwrap();
    println!("Disabling input buffering");
    flush_stdout();
    tio.c_lflag &= !(ICANON | ECHO);
    tcsetattr(STDIN_FILENO, TCSANOW, &tio).unwrap();
    tio
}

pub fn restore_input_buffering(termios: &Termios) {
    println!("Restoring input buffering");
    flush_stdout();
    tcsetattr(STDIN_FILENO, TCSANOW, termios).unwrap();
}

pub fn check_key() -> bool {
    let mut readfds: fd_set = unsafe { zeroed() };

    unsafe {
        FD_ZERO(&mut readfds);
        FD_SET(STDIN_FILENO as RawFd, &mut readfds);
    }

    let mut timeout = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };

    let result = unsafe {
        select(
            1,
            &mut readfds,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            &mut timeout,
        )
    };

    result != 0
}

pub fn handle_interrupt(_signal: c_int) {
    restore_input_buffering(&Termios::from_fd(STDIN_FILENO).unwrap());
    println!();
    std::process::exit(-2);
}
