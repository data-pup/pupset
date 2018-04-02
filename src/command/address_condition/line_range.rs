use command::address_condition::{
    Address,
    Condition,
    TwoAddressCondition,
    RangeBounds,
};

pub struct LineRange {
    min: RangeBounds,
    max: RangeBounds
}

impl Condition for LineRange {
    fn applies(&self, current_line: Address) -> bool {
        self.check_min(current_line) && self.check_max(current_line)
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

impl LineRange {
    fn check_min(&self, addr: Address) -> bool {
        match self.min.is_inclusive {
            true if  (self.min.val <= addr) != true => return false,
            false if (self.min.val < addr)  != true => return false,
            _ => { true }
        }
    }

    fn check_max(&self, addr: Address) -> bool {
        match self.max.is_inclusive {
            true if  (self.max.val >= addr) != true => return false,
            false if (self.max.val > addr)  != true => return false,
            _ => { true }
        }
    }
}

#[cfg(test)]
mod tests {
    use command::address_condition::line_range::*;

    #[test]
    fn min_works() {
        let range = LineRange::new(5, 10);
        assert_eq!(range.applies(4),  false);
        assert_eq!(range.applies(5),  true);
        assert_eq!(range.applies(9),  true);
        assert_eq!(range.applies(10), false);
    }
}