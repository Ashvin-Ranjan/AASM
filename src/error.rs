#[derive(Debug)]
pub enum AssemblerError {
    IncorrectInstructionPass,
    InvalidSyntaxError,
    InvalidRegisterError,
    InvalidInstructionError,
    InvalidScalarError,
    LiteralOverflowError,
    InvalidStringError,
}