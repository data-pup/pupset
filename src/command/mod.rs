pub mod address_condition;
pub mod commands;

mod line;
pub use command::line::Line;

mod parse;

use std::collections::HashMap;

use self::address_condition::{
    Condition,
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

    // fn from_args(args: Vec<String>) -> Option<Box<Self>>;
}

// ----------------------------------------------------------------------------
// pub struct CommandInfo {
//     argc: &'static [usize],
//     name: &'static str,
// }
