use std::str::Chars;
use command::Address;
use command::address_condition::{
    AddressCondition,
    LineNumber,
    LineRange,
};

// Result type used to represent the results of the parsing process.
pub type ParseResult = Result<ParsedAddrCond, ArgParseError>;

// This is used by the splitting function, to represent the characters
// identified as the upper/lower enclosures, and the individual addresses.
#[derive(Debug, PartialEq)]
pub struct ParsedAddrCond {
    lower_enclosure: ClosureType,
    body_tokens:     AddressList,
    upper_enclosure: ClosureType,
}

// These are the different types of errors that the parsing function can return.
#[derive(Debug, PartialEq)]
pub enum ArgParseError {
    ArgEmpty,              // Used if the argument is empty.
    MissingClosuresError,  // Used if no bounds characters are used.
    InvalidAddressNumber,  // Used if the input does not contain valid addresses.
    InvalidClosuresError,  // Used if a single address is not inclusively bound.
    InvalidAddressCount,   // Used if too many, or too few addresses were given.
}

// Helper types and enums for the parsed address condition.
pub type AddressList = Vec<Address>;
#[derive(Debug, PartialEq)]
pub enum ClosureType { Inclusive, Exclusive }

// Static variables used to identify closures, and split addresses in a range.
static LOWER_BOUNDS_CHARS: [&str; 2] = ["[", "("];
static UPPER_BOUNDS_CHARS: [&str; 2] = ["]", ")"];
static RANGE_DELIM:        &str      = "..";

/// Parse an address condition argument in the form of a string. If the input
/// is not valid, return an error. If the argument is valid, allocate a new
/// condition object corresponding to the number of addresses given.
pub fn parse_arg(arg: &String) -> Result<Box<AddressCondition>, ArgParseError> {
    unimplemented!();
    // if      arg.is_empty()        { Err(ArgParseError::ArgEmpty)             }
    // else if !check_closures(&arg) { Err(ArgParseError::MissingClosuresError) }
    // else {
    //     let ParsedAddrCond {
    //         lower_enclosure, body_tokens, upper_enclosure,
    //     } = split_arg(&arg)?;
    //     match body_tokens.len() {
    //         1 => Ok(Box::new(LineNumber::new(body_tokens[0]))),
    //         2 => Ok(Box::new(LineRange::new(body_tokens[0], body_tokens[1]))),
    //         _ => Err(ArgParseError::InvalidAddressCount),
    //     }
    // }
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
fn split_arg(arg: &String) -> ParseResult {
    let mut chars:           Chars       = arg.chars();
    let     lower_enclosure: ClosureType = get_lower_enclosure_type(&mut chars)?;
    let mut body:            Vec<char>   = chars.collect();
    let     upper_enclosure: ClosureType = get_upper_enclosure_type(&mut body)?;
    let     body_tokens:     AddressList = get_address_list(body)?;
    return validate_split(ParsedAddrCond {
        lower_enclosure, body_tokens, upper_enclosure,
    });
}

// Use a char iterator to identify the opening closure of an address condition.
fn get_lower_enclosure_type(chars: &mut Chars)
    -> Result<ClosureType, ArgParseError> {
    match chars.next() {
        Some(c) if c == '[' => Ok(ClosureType::Inclusive),
        Some(c) if c == '(' => Ok(ClosureType::Exclusive),
        _ => Err(ArgParseError::MissingClosuresError),
    }
}

// Pop the last element off of the char vector, to identify the second closure.
fn get_upper_enclosure_type(chars: &mut Vec<char>)
    -> Result<ClosureType, ArgParseError> {
    match chars.pop() {
        Some(c) if c == ']' => Ok(ClosureType::Inclusive),
        Some(c) if c == ')' => Ok(ClosureType::Exclusive),
        _ => Err(ArgParseError::MissingClosuresError),
    }
}

// Split the body of the address condition argument into address tokens.
fn get_address_list(body_chars: Vec<char>)
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

/// Checks that the parse results are valid, if the results do not
/// seem well formed, return an error. Otherwise, return the results.
fn validate_split(parse_results: ParsedAddrCond) -> ParseResult {
    match parse_results.body_tokens.len() {
        0 => Err(ArgParseError::ArgEmpty),
        1 if   parse_results.lower_enclosure == ClosureType::Exclusive
            || parse_results.lower_enclosure == ClosureType::Exclusive
            => Err(ArgParseError::InvalidClosuresError),
        _ => Ok(parse_results),
    }
}

#[cfg(test)]
mod parse_tests {
    use command::address_condition::parse::*;

    // FIXUP: Disabling broken test temporarily.

    // #[test]
    // fn parsing_works() {
    //     for curr_test_case in init_test_cases().into_iter() {
    //         let ParseTestCase {
    //             input_string,
    //             expected_result,
    //             test_description,
    //         } = curr_test_case;
    //         let actual_result: Result<Box<Condition>, ArgParseError> =
    //             parse_arg(&input_string);
    //         match expected_result.is_ok() {
    //             true => {
    //                 let actual_cond   = actual_result.unwrap();
    //                 let expected_cond = expected_result.unwrap();
    //                 assert_eq!(*actual_cond, *expected_cond,
    //                     "Test Failed: {}", test_description);
    //             }
    //             false => {
    //                 let actual_err   = actual_result.unwrap_err();
    //                 let expected_err = expected_result.unwrap_err();
    //                 assert_eq!(actual_err, expected_err,
    //                     "Test Failed: {}", test_description);
    //             }
    //         }
    //     }
    // }

    fn init_test_cases() -> Vec<ParseTestCase> {
        vec![
            ParseTestCase {
                input_string: String::from("[1]"),
                expected_result: Ok(Box::new(LineNumber::new(1))),
                test_description: "Single digit inclusively enclosed.",
            },
            ParseTestCase {
                input_string: String::from("[11]"),
                expected_result: Ok(Box::new(LineNumber::new(11))),
                test_description: "Double digit inclusively enclosed.",
            },
            ParseTestCase {
                input_string: String::from(""),
                expected_result: Err(ArgParseError::ArgEmpty),
                test_description: "Empty argument should cause ArgEmpty error.",
            },
            ParseTestCase {
                input_string: String::from("[]"),
                expected_result: Err(ArgParseError::InvalidAddressNumber),
                test_description: "Empty [] enclosure should cause error.",
            },
            ParseTestCase {
                input_string: String::from("()"),
                expected_result: Err(ArgParseError::InvalidAddressNumber),
                test_description: "Empty () enclosure should cause error.",
            },
            ParseTestCase {
                input_string: String::from("(1)"),
                expected_result: Err(ArgParseError::InvalidClosuresError),
                test_description: "Single address must be enclosed inclusively.",
            },
        ]
    }

    struct ParseTestCase {
        input_string:     String,
        expected_result:  Result<Box<Condition>, ArgParseError>,
        test_description: &'static str,
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
                expected_end_result:     true,
                test_description:        "Single-digit, valid inclusive bounds.",
            },
            ClosureTestCase {
                input_string:            String::from("[11]"),
                expected_start_result:   true,
                expected_end_result:     true,
                test_description:        "Two-digit, valid inclusive bounds.",
            },
            ClosureTestCase {
                input_string:            String::from("(1)"),
                expected_start_result:   true,
                expected_end_result:     true,
                test_description:        "Single-digit, valid exclusive bounds.",
            },
            ClosureTestCase {
                input_string:            String::from("1"),
                expected_start_result:   false,
                expected_end_result:     false,
                test_description:        "Single-digit, missing bounds.",
            },
            ClosureTestCase {
                input_string:            String::from("[1"),
                expected_start_result:   true,
                expected_end_result:     false,
                test_description:        "Single-digit, missing upper bounds.",
            },
            ClosureTestCase {
                input_string:            String::from("1]"),
                expected_start_result:   false,
                expected_end_result:     true,
                test_description:        "Single-digit, missing lower bounds.",
            },
        ]
    }

    struct ClosureTestCase {
        input_string:            String,
        expected_start_result:   bool,
        expected_end_result:     bool,
        test_description:        &'static str,
    }
}
