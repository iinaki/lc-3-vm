/// Custom error for the VM
#[derive(Debug)]
pub enum VmError {
    BadArgsLength(String),
    FailedToCreateTermios(String),
    FailedToSetAttrTermios(String),
    FailedToOpenFile(String),
    FailedToReadBigEndian(String),
    FailedToFlush(String),
    FailedToReadStdin(String),
    InvalidRegister(String),
}
