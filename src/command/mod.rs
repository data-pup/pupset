use std::convert::TryFrom;

mod address_condition;
pub use self::address_condition::{
    AddressCondition,
    AddressConditionParseError,
};

mod line;
pub use self::line::Line;

mod parse_error;
pub use self::parse_error::CommandParseError;

pub type Address = u32;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CommandType {
    Delete,
    Print,
}

#[derive(Debug, PartialEq)]
pub struct Command {
    comm: CommandType,
    cond: Option<AddressCondition>,
}

impl Command {
    /// Run the command for the given line.
    pub fn run(&self, line: Line) -> Line {
        if !self.should_run(&line) { return line; }
        match self.comm {
            CommandType::Print => unimplemented!(),
            CommandType::Delete => return Line {
                addr: line.addr, contents: String::new()
            },
        }
    }

    /// Internal helper, checks if conditions are met to run.
    fn should_run(&self, line: &Line) -> bool {
        match &self.cond {
            &Some(ref c) => c.applies(line.addr),
            &None    => true,
        }
    }
}

// Creates a command object from a vector of strings. (In progress)
impl TryFrom<Vec<String>> for Command {
    type Error = CommandParseError;

    fn try_from(args: Vec<String>) -> Result<Self, Self::Error> {

        if args.is_empty() { return Err(CommandParseError::EmptyCommand); }
        let comm: CommandType = match args[0].as_ref() {
            "delete" => CommandType::Delete,
            "print"  => CommandType::Print,
            _        => unimplemented!(),
        };
        return Ok(Self { comm, cond: None });
    }
}

#[cfg(test)]
mod command_tests {
    use command::*;

    #[test]
    fn command_keywords_identified_correctly() {
        let keywords_and_types = &[ // Keywords and expected command types.
            ("delete", CommandType::Delete),
            ("print" , CommandType::Print ),
        ];

        for &(keyword, expected_type) in keywords_and_types.iter() {
            let comm_args = vec![keyword.to_string(), String::from("(1)")];
            let comm = Command::try_from(comm_args).unwrap();
            assert_eq!(comm, Command { comm: expected_type, cond: None });
        }
    }
}
