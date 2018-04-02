use std::ops::Index;
use std::str::{Chars, FromStr};

use command::address_condition::{
    Address
};

pub type ParseResult = Result<AddressList, ArgParseError>;
pub type AddressList = Vec<Address>;
#[derive(Debug, PartialEq)]
pub enum ArgParseError {
    ArgEmpty,
    MissingClosuresError,
    InvalidAddressNumber,
}

struct ConditionTokens {
    lower_enclosure: String,
    addr_tokens:     Vec<String>,
    upper_enclosure: String,
}

static LOWER_BOUNDS_CHARS: [&str; 2] = ["[", "("];
static UPPER_BOUNDS_CHARS: [&str; 2] = ["]", ")"];
static RANGE_DELIM:        &str      = "..";

pub fn parse_arg(arg: &String) -> ParseResult {
    if arg.is_empty() { return Err(ArgParseError::ArgEmpty); }
    if !check_closures(&arg) {
        return Err(ArgParseError::MissingClosuresError);
    }
    Ok(vec![])
}

fn check_closures(arg: &String) -> bool {
    arg_starts_with_closure(&arg) && arg_ends_with_closure(&arg)
}

fn arg_starts_with_closure(arg: &String) -> bool {
    LOWER_BOUNDS_CHARS.iter()
        .map(|closure| arg.starts_with(closure))
        .fold(false, |acc, elem| acc || elem)
}

fn arg_ends_with_closure(arg: &String) -> bool {
    UPPER_BOUNDS_CHARS.iter()
        .map(|closure| arg.ends_with(closure))
        .fold(false, |acc, elem| acc || elem)
}

fn split_arg(arg: &String) -> ConditionTokens {
    let mut arg_clone: String = arg.clone();
    let mut body: Chars = arg_clone.chars();
    let lower_enclosure: String = body.next()
        .expect("Error identifying lower enclosure.")
        .to_string();
    let mut body_chars: Vec<char> = body.collect();
    let upper_enclosure: String = body_chars.pop()
        .expect("Error identifying upper enclosure.")
        .to_string();
    let body_string: String = body_chars
        .into_iter()
        .collect::<String>();
    let addr_tokens: Vec<String> = body_string
        .split(RANGE_DELIM)
        .map(|raw_str| String::from_str(raw_str).unwrap()) // FIXUP
        .collect();
    return ConditionTokens {
        lower_enclosure,
        addr_tokens,
        upper_enclosure,
    };
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
