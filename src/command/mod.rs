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


/// Command structure.
pub trait Command {
    // fn from_args(args: Vec<String>) -> Option<Box<Self>>;
    fn should_run(&self, curr_line: Address) -> bool;
    fn run(&self, line: Line) -> Line;
}

// ----------------------------------------------------------------------------
// pub struct CommandInfo {
//     argc: &'static [usize],
//     name: &'static str,
// }
