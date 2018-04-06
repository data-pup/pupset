/// Comamand parse errors.
#[derive(Debug)]
pub enum CommandParseError {
    EmptyCommand,
    InvalidArgCount,
    InvalidAddressCond,
    InvalidCommandName,
}
