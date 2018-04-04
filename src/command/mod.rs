pub mod address_condition;

mod line;
pub use command::line::Line;

use self::address_condition::{
    AddressCondition,
    LineNumber,
    parse_arg,
};

pub type Address = u32;

pub enum CommandType {
    Delete,
    Print,
}

pub struct Command {
    comm: CommandType,
    // cond:
}

impl Command {
    fn should_run(&self, curr_line: Address) -> bool {
        unimplemented!();
    }

    pub fn run(&self, line: Line) -> Line {
        if !self.should_run(line.addr) { return line; }
        match &self.comm {
            Print => unimplemented!(),
            Delete => return Line { addr: line.addr, contents: String::new() }
        }
    }
}

impl From<Vec<String>> for Command {
    fn from(s: Vec<String>) -> Self {
        unimplemented!();
    }
}
