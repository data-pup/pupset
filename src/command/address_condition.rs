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
                3 => AddressCondition::parse_step_range(
                                            &tokens[0], &tokens[1], &tokens[2]),
                _ => Err(AddressConditionParseError::InvalidArgument),
            }
        }
    }
}

// FromStr private helper methods.
impl AddressCondition {
    /// Parse a line number condition, and return the result.
    fn parse_line_number(arg: &String) -> ParseResult {
        let treated_arg: String = AddressCondition::replace_closures(arg);
        let addr: Address = AddressCondition::parse_addr(treated_arg)?;
        let cond = AddressCondition {vals: Values::LineNumber(addr)};
        return Ok(cond);
    }

    /// This function will parse and return a line range condition, given the
    /// minimum and maximum tokens. If an invalid closure or address is
    /// encountered, returns an error.
    fn parse_line_range(min_token: &String, max_token: &String) -> ParseResult {
        if min_token.len() < 2 || max_token.len() < 2 { // Check token lengths.
            return Err(AddressConditionParseError::InvalidLineNumber);
        }

        // First, split the min and max tokens into a closure/address tuple.
        let (min_split_i, max_split_i) = (1, max_token.len() - 1);
        let (min_closure_s, min_addr_s) = min_token.split_at(min_split_i);
        let (max_addr_s, max_closure_s) = max_token.split_at(max_split_i);

        // Parse the upper and lower bounds' closures and addresses.
        let min_inclusive = AddressCondition::is_min_inclusive(min_closure_s)?;
        let min = AddressCondition::parse_addr(min_addr_s.to_owned())?;
        let max_inclusive = AddressCondition::is_max_inclusive(max_closure_s)?;
        let max = AddressCondition::parse_addr(max_addr_s.to_owned())?;

        Ok(AddressCondition { // Return an address condition for the range.
            vals: Values::Range { min, min_inclusive, max, max_inclusive }
        })
    }

    /// Parse a step range, given the upper and lower bounds tokens, as well as
    /// the step size token. Returns a step range condition, or an error.
    fn parse_step_range(min_token: &String, step_token: &String,
                        max_token: &String) -> ParseResult {
        unimplemented!();
    }

    /// Parse a String into an Address, or return an InvalidLineNumber error.
    fn parse_addr(s: String) -> Result<Address, AddressConditionParseError> {
        match s.parse::<Address>() {
            Ok(addr) => Ok(addr),
            Err(_)   => Err(AddressConditionParseError::InvalidLineNumber),
        }
    }

    /// Remove the closures in a given String value.
    fn replace_closures(s: &String) -> String {
        return s // FIXUP? Not sure if there is a way to replace multiple chars.
            .replace("[", "").replace("(", "")
            .replace("]", "").replace(")", "");
    }

    /// Return true/false based on whether the lower bound closure is or is not
    /// inclusive. Returns an InvalidRangeClosure error if an unrecognized
    /// closure was given.
    fn is_min_inclusive(closure: &str)
        -> Result<bool, AddressConditionParseError> {
        match closure {
            s if s == "[" => Ok(true),
            s if s == "(" => Ok(false),
            _ => Err(AddressConditionParseError::InvalidRangeClosure),
        }
    }

    /// Return true/false based on whether the upper bound closure is or is not
    /// inclusive. Returns an InvalidRangeClosure error if an unrecognized
    /// closure was given.
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
        inputs:       &'static [&'static str],
        apply_checks: &'static [(Address, bool)],
        desc:         &'static str,
    }

    const TEST_CASES: &[ConditionParseTest] = &[
        ConditionParseTest { // Test that a line number condition works.
            inputs:        &["[10]", "(10)", "10"],
            apply_checks:  &[(9, false), (10, true), (11, false)],
            desc:          "Single digit (0) inclusively enclosed",
        },
        ConditionParseTest { // Test that a line range condition works.
            inputs:        &["[29..30]", "(28..31)", "[29..31)", "(28..30]"],
            apply_checks:  &[(28, false), (29, true), (30, true), (31, false)],
            desc:          "Line ranges with inclusive/exclusive bounds",
        },
        ConditionParseTest { // Test a simple step range condition.
            inputs:        &["[0..2..6)"],
            apply_checks:  &[(0, true), (1, false), (2, true), (3, false),
                             (4, true), (5, false), (6, false)],
            desc:          "Simple step range condition [0..2..6)",
        }
    ];

    #[test]
    fn line_numbers_parse_correctly() {
        TEST_CASES.iter().for_each(
            |&ConditionParseTest {      // Destructure each test case.
                inputs, apply_checks, desc,
            }: &ConditionParseTest| {   // Run the test for each input.
                for arg in inputs.iter() {
                    let output = arg.parse::<AddressCondition>();
                    if output.is_ok() { // Assert the check function passes.
                        let cond: AddressCondition = output.unwrap();
                        assert!(check_applies_results(cond, apply_checks),
                            "Test Failed: [{}]", desc);
                    }
                }
        });
    }

    #[test]
    fn temp() {
        let s = "hello";
        let (a, b) = s.split_at(s.len() - 1);
        assert_eq!(a, "hell");
        assert_eq!(b, "o");
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
