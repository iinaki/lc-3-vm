use libc::STDIN_FILENO;
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
