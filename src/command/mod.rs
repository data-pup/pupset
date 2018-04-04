mod address_condition;
mod line;

use self::address_condition::AddressCondition;
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
    fn should_run(&self, line: &Line) -> bool {
        return self.cond.applies(line.addr);
    }

    pub fn run(&self, line: Line) -> Line {
        if !self.should_run(&line) { return line; }
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
