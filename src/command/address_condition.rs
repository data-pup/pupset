use std::str::FromStr;
use command::Address;

enum Values {
    LineNumber(Address),
    Range {
        min:           Address,
        max:           Address,
        min_inclusive: bool,
        max_inclusive: bool,
    },
    StepRange {
        min:           Address,
        max:           Address,
        min_inclusive: bool,
        max_inclusive: bool,
        step:          Address,
    },
}

#[derive(Debug)]
pub enum ConditionParseErrors {
    ArgEmpty,
}

pub struct AddressCondition {
    vals: Values,
}

impl AddressCondition {
    pub fn applies(&self, addr: Address) -> bool {
        match self.vals {
            Values::LineNumber(n) => addr == n,
            _ => unimplemented!(),
        }
    }
}

impl FromStr for AddressCondition {
    type Err = ConditionParseErrors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(AddressCondition { vals: Values::LineNumber(0) })
    }
}

#[cfg(test)]
mod condition_tests {
    use command::address_condition::*;

    #[test]
    fn it_works() {
        let input = "[0]";
        let comm = input.parse::<AddressCondition>().unwrap();
        assert_eq!(comm.applies(0), true);
        assert_eq!(comm.applies(1), false);
    }
}
