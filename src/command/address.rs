type Address = u32;

pub trait AddressCondition {
    fn applies(&self, current_line: Address) -> bool;
}

pub trait OneAddressCondition {
    fn new(addr: Address) -> Self;
}

pub trait TwoAddressCondition {
    fn new(a: Address, b: Address) -> Self;
}

pub struct LineNumber {
    n: Address,
}

impl OneAddressCondition for LineNumber {
    fn new(addr: Address) -> Self {
        LineNumber { n:addr }
    }
}

impl AddressCondition for LineNumber {
    fn applies(&self, current_line: Address) -> bool { self.n == current_line }
}
