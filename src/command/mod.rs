mod address_condition;
pub use self::address_condition::{
    AddressCondition,
    AddressConditionParseError,
};

mod line;
pub use self::line::Line;

pub type Address = u32;

#[derive(Debug, PartialEq)]
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
    pub fn run(&self, line: Line) -> Line {
        if !self.should_run(&line) { return line; }
        match self.comm {
            CommandType::Print => unimplemented!(),
            CommandType::Delete => return Line {
                addr: line.addr, contents: String::new()
            },
        }
    }

    fn should_run(&self, line: &Line) -> bool {
        match &self.cond {
            &Some(ref c) => c.applies(line.addr),
            &None    => true,
        }
    }
}

// TODO: ...
impl From<Vec<String>> for Command {
    fn from(s: Vec<String>) -> Self {
        let comm: CommandType = match s[0].as_ref() {
            "delete" => CommandType::Delete,
            "print"  => CommandType::Print,
            _        => unimplemented!(),
        };
        Self { comm, cond: None }
    }
}

#[cfg(test)]
mod command_tests {
    use command::*;

    #[test]
    fn placeholder_command_test() {
        let delete_args = vec![String::from("delete"), String::from("[1]")];
        let delete_comm = Command::from(delete_args);
        assert_eq!(delete_comm, Command { comm: CommandType::Delete, cond: None });

        let print_args = vec![String::from("print"), String::from("[1]")];
        let print_comm = Command::from(print_args);
        assert_eq!(print_comm, Command { comm: CommandType::Print, cond: None });
    }
}
