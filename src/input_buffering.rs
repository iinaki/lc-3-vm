use std::mem::zeroed;
use std::os::unix::io::RawFd;

// struct termios original_tio;
use termios::*;
use libc::{c_int, fd_set, select, timeval, FD_SET, FD_ZERO, STDIN_FILENO};

// void disable_input_buffering()
// {
//     tcgetattr(STDIN_FILENO, &original_tio);
//     struct termios new_tio = original_tio;
//     new_tio.c_lflag &= ~ICANON & ~ECHO;
//     tcsetattr(STDIN_FILENO, TCSANOW, &new_tio);
// }

pub fn disable_input_buffering(termios: &mut Termios) {
    match tcgetattr(STDIN_FILENO, termios){
        Ok(_) => (),
        Err(e) => {
            println!("Error getting terminal attributes: {}", e);
            std::process::exit(2);
        }
    }
    termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(0, TCSANOW, termios).unwrap();
}

// void restore_input_buffering()
// {
//     tcsetattr(STDIN_FILENO, TCSANOW, &original_tio);
// }

pub fn restore_input_buffering(termios: &Termios) {
    tcsetattr(STDIN_FILENO, TCSANOW, termios).unwrap();
}

// uint16_t check_key()
// {
//     fd_set readfds;
//     FD_ZERO(&readfds);
//     FD_SET(STDIN_FILENO, &readfds);

//     struct timeval timeout;
//     timeout.tv_sec = 0;
//     timeout.tv_usec = 0;
//     return select(1, &readfds, NULL, NULL, &timeout) != 0;
// }

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

    let result = unsafe { select(1, &mut readfds, std::ptr::null_mut(), std::ptr::null_mut(), &mut timeout) };

    result != 0
}

// void handle_interrupt(int signal)
// {
//     restore_input_buffering();
//     printf("\n");
//     exit(-2);
// }
pub fn handle_interrupt(_signal: c_int) {
    restore_input_buffering(&Termios::from_fd(STDIN_FILENO).unwrap());
    println!();
    std::process::exit(-2);
}

// extern "C" fn handle_interrupt(signal: c_int) {
//     // Restore terminal settings and exit
//     let mut termios = Termios::from_fd(STDIN_FILENO).unwrap();
//     termios.restore_input_buffering().unwrap();
//     println!();
//     exit(-2);
// }
