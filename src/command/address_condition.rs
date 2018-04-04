use std::str::FromStr;
use command::Address;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum AddressConditionParseError {
    ArgEmpty,
    InvalidArgument,
    InvalidLineNumber,
    InvalidRangeClosure,
    StringParseError,
}

type ParseResult = Result<AddressCondition, AddressConditionParseError>;

#[derive(Debug, PartialEq)]
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

    fn from_str(s: &str) -> ParseResult {
        let cond_tokens: Result<Vec<String>, _> =
            s.split("..") // Split the given string apart.
            .map(String::from_str)
            .collect();
        if cond_tokens.is_err() { // Return an error if parsing failed.
            return Err(AddressConditionParseError::StringParseError);
        } else { // Otherwise, unwrap the result and form an address condition.
            let tokens = cond_tokens.unwrap();
            return match tokens.len() {
                1 => AddressCondition::parse_line_number(&tokens[0]),
                2 => AddressCondition::parse_line_range(&tokens[0], &tokens[1]),
                3 => unimplemented!(),
                _ => Err(AddressConditionParseError::InvalidArgument),
            }
        }
    }
}

// FromStr private helper methods.
impl AddressCondition {
    fn parse_line_number(arg: &String) -> ParseResult {
        let treated_arg: String = AddressCondition::replace_closures(arg);
        let addr: Address = AddressCondition::parse_addr(treated_arg)?;
        let cond = AddressCondition {vals: Values::LineNumber(addr)};
        return Ok(cond);
    }

    fn parse_line_range(min_token: &String, max_token: &String)
        -> ParseResult {
        let (min_closure_s, min_addr_s) = min_token.split_at(1);
        let (max_closure_s, max_addr_s) = max_token.split_at(1);
        let min_inclusive = AddressCondition::is_min_inclusive(min_closure_s)?;
        let min = AddressCondition::parse_addr(min_addr_s.to_owned())?;
        let max_inclusive = AddressCondition::is_max_inclusive(max_closure_s)?;
        let max = AddressCondition::parse_addr(max_addr_s.to_owned())?;
        let cond = AddressCondition { vals: Values::Range {
            min, min_inclusive,
            max, max_inclusive,
        }};
        return Ok(cond);
    }

    fn parse_addr(s: String) -> Result<Address, AddressConditionParseError> {
        match s.parse::<Address>() {
            Ok(addr) => Ok(addr),
            Err(_)   => Err(AddressConditionParseError::InvalidLineNumber),
        }
    }

    fn replace_closures(s: &String) -> String {
        return s
            .replace("[", "").replace("(", "")
            .replace("]", "").replace(")", "");
    }

    fn is_min_inclusive(closure: &str)
        -> Result<bool, AddressConditionParseError> {
        match closure {
            s if s == "[" => Ok(true),
            s if s == "(" => Ok(false),
            _ => Err(AddressConditionParseError::InvalidRangeClosure),
        }
    }

    fn is_max_inclusive(closure: &str)
        -> Result<bool, AddressConditionParseError> {
        match closure {
            s if s == "]" => Ok(true),
            s if s == ")" => Ok(false),
            _ => Err(AddressConditionParseError::InvalidRangeClosure),
        }
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
        inputs:          &'static [&'static str],
        expected:        ParseResult,
        check_fn:        Option<fn(AddressCondition) -> bool>,
        desc:            &'static str,
    }

    const LINE_NUMBER_TESTS: &[LineNumberTest] = &[
        LineNumberTest {
            inputs: &["[1]", "(1)", "1"],
            expected: Ok(AddressCondition {
                vals: Values::LineNumber(1),
            }),
            check_fn: Some(
                |cond: AddressCondition| -> bool {
                    [false, true, false]
                        .into_iter().enumerate()
                        .map(|(i, res)| (cond.applies(i as Address), res))
                        .fold(true, |res, (actual, expected)| res && (actual == *expected))
                }
            ),
            desc: "Single digit (0) inclusively enclosed",
        }
    ];

    #[test]
    fn line_numbers_parse_correctly() {
        LINE_NUMBER_TESTS.iter().for_each(
            |&LineNumberTest {    // Destructure each test case.
                inputs, ref expected, check_fn, desc,
            }: &LineNumberTest| { // Run the test for each input.
                for arg in inputs.iter() {
                    let output = arg.parse::<AddressCondition>();
                    assert_eq!(output, *expected, "Test Failed: [{}]", desc);
                    if output.is_ok() { // Assert that the check function passes.
                        let (cond, check) = (output.unwrap(), check_fn.unwrap());
                        assert_eq!(check(cond), true);
                    }
                }
        });
    }
}
