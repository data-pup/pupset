mod line_number;
mod line_range;

pub use self::line_number::LineNumber;
pub use self::line_range::{LineRange, RangeBounds};

pub type Address = u32;

pub trait Condition {
    fn applies(&self, current_line: Address) -> bool;
}

pub trait OneAddressCondition : Condition {
    fn new(addr: Address) -> Self;
}

pub trait TwoAddressCondition : Condition {
    fn new(a: Address, b: Address) -> Self;
}

pub enum AddressCondition {OneAddressCondition, TwoAddressCondition}

// pub fn parse_arg(arg: String) -> Ad
