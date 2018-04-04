mod address_condition;
pub use self::address_condition::{
    AddressCondition,
    AddressConditionParseError,
};

mod line;
pub use self::line::Line;

pub type Address = u32;

pub enum CommandType {
    Delete,
    Print,
}

pub struct Command {
    comm: CommandType,
    cond: AddressCondition,
}

impl Command {
    pub fn run(&self, line: Line) -> Line {
        if !self.should_run(&line) { return line; }
        match &self.comm {
            Print => unimplemented!(),
            Delete => return Line { addr: line.addr, contents: String::new() }
        }
    }

    fn should_run(&self, line: &Line) -> bool {
        return self.cond.applies(line.addr);
    }
}

impl From<Vec<String>> for Command {
    fn from(s: Vec<String>) -> Self {
        unimplemented!();
    }
}
