mod line_number;
mod line_range;
mod parse;

pub use self::line_number::LineNumber;
pub use self::line_range::LineRange;
pub use self::parse::parse_arg;

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

#[derive(Clone)]
pub struct RangeBounds { val: Address, is_inclusive: bool }
