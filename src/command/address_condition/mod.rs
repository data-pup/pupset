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

#[derive(Clone)]
pub struct RangeBounds { val: Address, is_inclusive: bool }
