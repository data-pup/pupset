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
            Values::Range { min, min_inclusive, max, max_inclusive } => {
                let within_lower_bound: bool =
                    if min_inclusive { min <= addr }
                    else             { min <  addr };
                if within_lower_bound {
                    return if max_inclusive { addr <= max }
                           else             { addr <  max };
                } else { return false; }
            }
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
        // First, split the min and max tokens into a closure/address tuple.
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
mod parse_tests {
    use command::{
        Address,
        AddressCondition,
        AddressConditionParseError,
    };
    use command::address_condition::Values;

    type ParseResult = Result<AddressCondition, AddressConditionParseError>;
    struct ConditionParseTest {
        inputs:          &'static [&'static str],
        expected:        ParseResult,
        apply_checks: &'static [(Address, bool)],
        desc:            &'static str,
    }

    const TEST_CASES: &[ConditionParseTest] = &[
        ConditionParseTest {
            inputs: &["[1]", "(1)", "1"],
            expected: Ok(AddressCondition {
                vals: Values::LineNumber(1),
            }),
            apply_checks:  &[(0, false), (1, true), (2, false)],
            desc: "Single digit (0) inclusively enclosed",
        },
        ConditionParseTest {
            inputs: &["[1..3]"],
            expected: Ok(AddressCondition {
                vals: Values::Range{
                    min: 1, min_inclusive: true,
                    max: 3, max_inclusive: true,
                },
            }),
            apply_checks:  &[
                (0, false), (1, true), (2, true), (3, true), (4, false)
            ],
            desc: "Range [1..3]",
        }
    ];

    #[test]
    fn line_numbers_parse_correctly() {
        TEST_CASES.iter().for_each(
            |&ConditionParseTest {    // Destructure each test case.
                inputs, ref expected, apply_checks, desc,
            }: &ConditionParseTest| { // Run the test for each input.
                for arg in inputs.iter() {
                    let output = arg.parse::<AddressCondition>();
                    assert_eq!(output, *expected, "Test Failed: [{}]", desc);
                    if output.is_ok() { // Assert that the check function passes.
                        let cond: AddressCondition = output.unwrap();
                        assert!(check_applies_results(cond, apply_checks),
                            "Test Failed: [{}]", desc);
                    }
                }
        });
    }

    fn check_applies_results(cond: AddressCondition,
                             addrs_and_results: &[(Address, bool)])
        -> bool {
        addrs_and_results.iter()
            .map(|&(addr, res)| (cond.applies(addr), res))
            .fold(true, |res, (actual, expected)| -> bool {
                    res && (actual == expected)
            })
    }
}
