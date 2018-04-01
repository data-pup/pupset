pub mod line_number;
pub mod line_range;

pub type Address = u32;

pub trait AddressCondition {
    fn applies(&self, current_line: Address) -> bool;
}

pub trait OneAddressCondition {
    fn new(addr: Address) -> Self;
}

pub trait TwoAddressCondition {
    fn new(a: Address, b: Address) -> Self;
}
