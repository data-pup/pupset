/// Comamand parse errors.
#[derive(Debug, PartialEq)]
pub enum CommandParseError {
    EmptyCommand,
    InvalidArgCount,
    InvalidAddressCond,
    InvalidCommandName,
}
