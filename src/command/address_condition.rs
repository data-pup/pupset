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
pub enum AddressConditionParseError {
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
    type Err = AddressConditionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(AddressCondition { vals: Values::LineNumber(0) })
    }
}

#[cfg(test)]
mod line_number_parse_tests {
    use command::{
        Address,
        AddressCondition,
        AddressConditionParseError,
    };
    use command::address_condition::Values;

    type ParseResult = Result<AddressCondition, AddressConditionParseError>;
    struct LineNumberTest {
        input:           &'static str,
        addr:            Option<Address>,
        expected: ParseResult,
        desc:            &'static str,
    }

    const LINE_NUMBER_TESTS: &[LineNumberTest] = &[
        LineNumberTest {
            input: "[0]",
            addr: Some(0),
            expected: Ok(AddressCondition {
                vals: Values::LineNumber(0),
            }),
            desc: "Single digit (0) inclusively enclosed",
        }
    ];

    #[test]
    fn it_works() {
        LINE_NUMBER_TESTS.iter().for_each(
            |&LineNumberTest { // Destructure each test input.
                input, addr, ref expected, desc,
            }: &LineNumberTest| {
                println!("Hello! {}", desc);
        });
    }

}