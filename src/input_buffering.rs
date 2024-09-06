use libc::STDIN_FILENO;
use termios::*;

use crate::{utils::flush_stdout, vm_error::VmError};

/// Disables input buffering for the terminal.
///
/// # Returns
///
/// Returns a Result with the current `Termios` struct containing the terminal's settings
/// before modification, which can be used later to restore the original settings, if everything went well.
/// Otherwise, an `VmError` is returned.
///
pub fn disable_input_buffering() -> Result<Termios, VmError> {
    let mut tio = Termios::from_fd(STDIN_FILENO)
        .map_err(|e| VmError::FailedToCreateTermios(e.to_string()))?;

    println!("Disabling input buffering");
    flush_stdout()?;

    tio.c_lflag &= !(ICANON | ECHO);
    tcsetattr(STDIN_FILENO, TCSANOW, &tio)
        .map_err(|e| VmError::FailedToSetAttrTermios(e.to_string()))?;
    Ok(tio)
}

/// Restores input buffering for the terminal.
///
/// This function restores the terminal's input buffering to its original state
/// using the provided `Termios` struct. It is typically called after
/// `disable_input_buffering` to revert the terminal to its previous settings.
///
/// # Parameters
///
/// - `termios`: A reference to the `Termios` struct with the original
///   terminal settings to be restored.
///
/// # Returns
///
/// Returns `Ok(())` if the terminal settings were successfully restored. And an `VmError` otherwise.
///
pub fn restore_input_buffering(termios: &Termios) -> Result<(), VmError> {
    println!("Restoring input buffering");
    flush_stdout()?;
    tcsetattr(STDIN_FILENO, TCSANOW, termios)
        .map_err(|e| VmError::FailedToSetAttrTermios(e.to_string()))
}
