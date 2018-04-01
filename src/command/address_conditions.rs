use command::address_traits::{
    Address,
    AddressCondition,
    OneAddressCondition,
    // TwoAddressCondition,
};

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
