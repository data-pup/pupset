use std::fmt::Debug;

use command::Address;

mod line_number;
mod line_range;
mod parse;

pub use self::line_number::LineNumber;
pub use self::line_range::LineRange;
pub use self::parse::parse_arg;

pub struct AddressCondition {
}

impl AddressCondition {
    fn applies(&self, current_line: Address) -> bool {
        unimplemented!();
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RangeBounds { val: Address, is_inclusive: bool }
