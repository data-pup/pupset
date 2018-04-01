use command::address_condition::{
    Address,
    AddressCondition,
    TwoAddressCondition,
};

pub struct LineRange {
    min: RangeBounds,
    max: RangeBounds
}

impl AddressCondition for LineRange {
    fn applies(&self, current_line: Address) -> bool {
        match self.min.is_inclusive {
            true if  (self.min.val <= current_line) != true => return false,
            false if (self.min.val < current_line)  != true => return false,
            _ => { }
        }
        true
    }
}

impl TwoAddressCondition for LineRange {
    fn new(a: Address, b: Address) -> Self {
        Self {
            min: RangeBounds { val: a, is_inclusive: true  },
            max: RangeBounds { val: b, is_inclusive: false },
        }
    }
}

#[derive(Clone)]
pub struct RangeBounds { val: Address, is_inclusive: bool }

#[cfg(test)]
mod tests {
    use command::address_condition::line_range::*;

    #[test]
    fn min_works() {
        let range = LineRange::new(5, 10);
        assert_eq!(range.applies(4),  false);
        assert_eq!(range.applies(5),  true);
        assert_eq!(range.applies(9),  true);
        // assert_eq!(range.applies(10), false);
    }
}