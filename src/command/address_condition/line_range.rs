use command::Address;
use command::address_condition::{
    AddressCondition,
    RangeBounds,
};

#[derive(Debug, PartialEq)]
pub struct LineRange {
    min: RangeBounds,
    max: RangeBounds
}

impl LineRange {
    pub fn new(a: Address, b: Address) -> Self {
        Self {
            min: RangeBounds { val: a, is_inclusive: true  },
            max: RangeBounds { val: b, is_inclusive: false },
        }
    }
}

// impl AddressCondition for LineRange {
//     fn applies(&self, current_line: Address) -> bool {
//         self.check_min(current_line) && self.check_max(current_line)
//     }
// }

impl LineRange {
    fn check_min(&self, addr: Address) -> bool {
        match self.min.is_inclusive {
            true  if (self.min.val <= addr) != true => return false,
            false if (self.min.val < addr)  != true => return false,
            _ => { true }
        }
    }

    fn check_max(&self, addr: Address) -> bool {
        match self.max.is_inclusive {
            true  if (self.max.val >= addr) != true => return false,
            false if (self.max.val > addr)  != true => return false,
            _ => { true }
        }
    }
}