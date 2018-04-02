// use std::
use std::ops::Index;
use std::str::{Chars, FromStr};

use command::address_condition::{
    Address,
    // RangeBounds,
};

// Result type used to represent the results of the parsing process.
pub type ParseResult = Result<AddressList, ArgParseError>;
pub type AddressList = Vec<Address>;
#[derive(Debug, PartialEq)]
pub enum ArgParseError {
    ArgEmpty,              // Used if the argument is empty.
    MissingClosuresError,  // Used if no bounds characters are used.
    InvalidAddressNumber,  // Used if the input does not contain valid addresses.
    StringParseError,      // Used if tokens could not be used to form a String.
}

// This represents the type of closure found at the boundary of an argument.
enum ClosureType { Inclusive, Exclusive }

// This is used by the splitting function, to represent the characters
// identified as the upper/lower enclosures, and the individual addresses.
struct ConditionTokens {
    lower_enclosure: ClosureType,
    body_tokens:     AddressList,
    upper_enclosure: ClosureType,
}

// Static variables used to identify closures, and split addresses in a range.
static LOWER_BOUNDS_CHARS: [&str; 2] = ["[", "("];
static UPPER_BOUNDS_CHARS: [&str; 2] = ["]", ")"];
static RANGE_DELIM:        &str      = "..";

// Parse an address condition argument.
pub fn parse_arg(arg: &String) -> ParseResult {
    if arg.is_empty() { return Err(ArgParseError::ArgEmpty); }
    if !check_closures(&arg) {
        return Err(ArgParseError::MissingClosuresError);
    }
    Ok(vec![])
}

// Check that an address condition begins and ends with valid closures.
fn check_closures(arg: &String) -> bool {
    arg_starts_with_closure(&arg) && arg_ends_with_closure(&arg)
}

// Check that an address condition argument starts with a valid closure.
fn arg_starts_with_closure(arg: &String) -> bool {
    LOWER_BOUNDS_CHARS.iter()
        .map(|closure| arg.starts_with(closure))
        .fold(false, |acc, elem| acc || elem)
}

// Check that an address condition argument ends with a valid closure.
fn arg_ends_with_closure(arg: &String) -> bool {
    UPPER_BOUNDS_CHARS.iter()
        .map(|closure| arg.ends_with(closure))
        .fold(false, |acc, elem| acc || elem)
}

// Splits an argument string into separate tokens, or returns an error.
fn split_arg(arg: &String) -> Result<ConditionTokens, ArgParseError> {
    let mut arg_chars: Chars = arg.chars();
    let lower_enclosure: String = get_lower_enclosure_token(&mut arg_chars)?;
    let mut body_chars: Vec<char> = arg_chars.collect();
    let upper_enclosure: String = get_upper_enclosure_token(&mut body_chars)?;
    let body_tokens: AddressList = get_condition_body_tokens(body_chars)?;
    Ok(ConditionTokens {lower_enclosure, body_tokens, upper_enclosure})
}

// Use a char iterator to identify the opening closure of an address condition.
fn get_lower_enclosure_token(chars: &mut Chars)
    -> Result<String, ArgParseError> {
    match chars.next() {
        Some(c) => Ok(c.to_string()),
        None    => Err(ArgParseError::MissingClosuresError),
    }
}

// Pop the last element off of the char vector, to identify the second closure.
fn get_upper_enclosure_token(chars: &mut Vec<char>)
    -> Result<String, ArgParseError> {
    match chars.pop() {
        Some(c) => Ok(c.to_string()),
        None    => Err(ArgParseError::MissingClosuresError),
    }
}

// Split the body of the address condition argument into address tokens.
fn get_condition_body_tokens(body_chars: Vec<char>)
    -> Result<AddressList, ArgParseError> {
    let body_string: String = body_chars.into_iter().collect::<String>();
    let parse_results: Result<AddressList, _> =
        body_string.split(RANGE_DELIM)
        .map(|s: &str| s.parse::<Address>())
        .collect();
    match parse_results {
        Ok(tokens) => Ok(tokens),
        Err(_)     => Err(ArgParseError::InvalidAddressNumber),
    }
}

#[cfg(test)]
mod parse_tests {
    use command::address_condition::parse::*;

    #[test]
    fn empty_string_returns_err() {
        let actual_result:   ParseResult = parse_arg(&String::from(""));
        let expected_result: ParseResult = Err(ArgParseError::ArgEmpty);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn parse_line_number() {
        let arg = String::from("[10]");
        let actual_result:   ParseResult = parse_arg(&arg);
        let expected_result: ParseResult = Ok(vec![10]);
        assert_eq!(actual_result, expected_result);
    }
}

#[cfg(test)]
mod cond_closures_tests {
    use command::address_condition::parse::*;

    #[test]
    fn test_arg_starts_with_closure() {
        let test_cases: Vec<ClosureTestCase> = init_test_cases();
        let run_test = |test: &ClosureTestCase| {
            let arg: &String = &test.input_string;
            let expected_result: &bool = &test.expected_start_result;
            let test_desc: &'static str = &test.test_description;
            let actual_result: bool = arg_starts_with_closure(arg);
            assert_eq!(actual_result, *expected_result,
                "Test failed: {}", test_desc);
        };
        test_cases.iter().for_each(|test| run_test(test));
    }

    #[test]
    fn test_arg_ends_with_closure() {
        let test_cases: Vec<ClosureTestCase> = init_test_cases();
        let run_test = |test: &ClosureTestCase| {
            let arg: &String = &test.input_string;
            let expected_result: &bool = &test.expected_end_result;
            let test_desc: &'static str = &test.test_description;
            let actual_result: bool = arg_ends_with_closure(arg);
            assert_eq!(actual_result, *expected_result,
                "Test failed: {}", test_desc);
        };
        test_cases.iter().for_each(|test| run_test(test));
    }

    fn init_test_cases() -> Vec<ClosureTestCase> {
        vec![
            ClosureTestCase {
                input_string:            String::from("[1]"),
                expected_start_result:   true,
                expected_body_tokens:    vec!["1"],
                expected_address_values: vec![1],
                expected_end_result:     true,
                test_description:        "Single-digit, valid inclusive bounds.",
            },
            ClosureTestCase {
                input_string:            String::from("[11]"),
                expected_start_result:   true,
                expected_body_tokens:    vec!["11"],
                expected_address_values: vec![11],
                expected_end_result:     true,
                test_description:        "Two-digit, valid inclusive bounds.",
            },
            ClosureTestCase {
                input_string:            String::from("(1)"),
                expected_start_result:   true,
                expected_body_tokens:    vec!["1"],
                expected_address_values: vec![1],
                expected_end_result:     true,
                test_description:        "Single-digit, valid exclusive bounds.",
            },
            ClosureTestCase {
                input_string:            String::from("1"),
                expected_start_result:   false,
                expected_body_tokens:    vec!["1"],
                expected_address_values: vec![1],
                expected_end_result:     false,
                test_description:        "Single-digit, missing bounds.",
            },
            ClosureTestCase {
                input_string:            String::from("[1"),
                expected_start_result:   true,
                expected_body_tokens:    vec!["1"],
                expected_address_values: vec![1],
                expected_end_result:     false,
                test_description:        "Single-digit, missing upper bounds.",
            },
            ClosureTestCase {
                input_string:            String::from("1]"),
                expected_start_result:   false,
                expected_body_tokens:    vec!["1"],
                expected_address_values: vec![1],
                expected_end_result:     true,
                test_description:        "Single-digit, missing lower bounds.",
            },
        ]
    }

    struct ClosureTestCase {
        input_string:            String,
        expected_start_result:   bool,
        expected_body_tokens:    Vec<&'static str>,
        expected_address_values: AddressList,
        expected_end_result:     bool,
        test_description:        &'static str,
    }
}
