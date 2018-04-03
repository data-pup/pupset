pub mod address_condition;
pub mod commands;
mod parse;

use std::collections::HashMap;

use self::address_condition::{
    Address,
    Condition,
    OneAddressCondition,
    LineNumber,
    parse_arg,
};

pub trait Command {
    fn from_args(args: Vec<String>) -> Option<Box<Self>>;
    fn should_run(&self, curr_line: Address) -> bool;
    fn run(&self, line: Line) -> Line;
}

pub struct CommandInfo {
    argc: &'static [usize],
    name: &'static str,
}

// pub enum CommandParseError {
//     InvalidArgCount,
//     InvalidAddressCond,
//     InvalidCommandName,
// }

#[derive(Clone, Debug, PartialEq)]
pub struct Line {
    contents: String,
    addr:     Address,
}
