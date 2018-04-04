use std::fmt::Debug;

use command::Address;

mod line_number;
mod line_range;
mod parse;

pub use self::line_number::LineNumber;
pub use self::line_range::LineRange;
pub use self::parse::parse_arg;

pub trait Condition : Debug {
    fn applies(&self, current_line: Address) -> bool;
}

#[derive(Clone, Debug, PartialEq)]
pub struct RangeBounds { val: Address, is_inclusive: bool }
