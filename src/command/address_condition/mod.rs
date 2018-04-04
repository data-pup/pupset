use std::fmt::Debug;

use command::Address;

mod line_number;
mod line_range;
mod parse;

pub use self::line_number::LineNumber;
pub use self::line_range::LineRange;
pub use self::parse::parse_arg;

struct Bounds {
    val:          Address,
    is_inclusive: bool,
}

pub struct AddressCondition {
    min:  Option<Bounds>,
    max:  Option<Bounds>,
    step: Option<Address>,
}

impl AddressCondition {
    fn applies(&self, addr: Address) -> bool {
        // First, check if the value is less than or equal to the min.
        let lower_bound = if self.min.is_inclusive { self.min.val    }
                           else                     { self.min.val + 1};
        if      addr <  lower_bound { return false; }
        else if addr == lower_bound { return true;  }
        else {  // If the address is greater than the lower bounds, check
                // if the value is greater than the upper bound.
            match self.max {
                None => { return false; },
                Some(_) => {
                    unimplemented!(); // FIXUP: This code is getting..... uh.
                }
            }
        }
    }
}

impl From<String> for AddressCondition {
    fn from(s: String) -> Self {
        AddressCondition {
            min: Bounds { val: 0, is_inclusive: true },
            max: None,
            step: None,
        }
    }
}

#[cfg(test)]
mod condition_tests {
    use command::address_condition::*;

    #[test]
    fn it_works() {
        assert!(true);
    }

    struct CondParseTestCase {
        input: &'static str,
        expected: AddressCondition,
    }
}
