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

pub struct AddressCondition {
    vals: Values,
}

impl AddressCondition {
    fn applies(&self, addr: Address) -> bool {
        match values {
            Value::LineNumber(n) => addr == n,
            _ => unimplemented!(),
        }
    }
}

impl From<String> for AddressCondition {
    fn from(s: String) -> Self {
        AddressCondition { vals: Values::LineNumber(0) }
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
