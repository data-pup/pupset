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
    // fn new(args: Vec<String>) -> Self; // TODO ...
    fn should_run(&self, curr_line: Address) -> bool;
    fn run(&self, line: Line) -> Line;
}

pub struct Line {
    contents: String,
    addr:     Address,
}
