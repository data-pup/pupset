use std::fmt::Debug;

use command::Address;

mod line_number;
mod line_range;
mod parse;

pub use self::line_number::LineNumber;
pub use self::line_range::LineRange;
pub use self::parse::parse_arg;

pub struct AddressCondition {
    min:  Address,
    max:  Option<Address>,
    step: Option<Address>,
}

impl AddressCondition {
    fn applies(&self, addr: Address) -> bool {
        if      addr <  self.min { false }
        else if addr == self.min { true  }
        else {
            unimplemented!();
        }
    }
}

impl From<String> for AddressCondition {
    fn from(s: String) -> Self {
        unimplemented!();
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RangeBounds { val: Address, is_inclusive: bool }
