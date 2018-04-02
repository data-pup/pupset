use command::address_condition::{
    Address
};

pub type ParseResult = Result<AddressList, ParseError>;
pub type AddressList = Vec<Address>;
#[derive(Debug, PartialEq)]
pub enum ParseError {
    ArgEmpty,
    MissingClosuresError,
    InvalidLowerBounds,
    InvalidUpperBounds,
    InvalidAddressNumber,
}

pub fn parse_arg(arg: &String) -> ParseResult {
    if arg.is_empty() { return Err(ParseError::ArgEmpty); }
    if !cond_closures::arg_starts_with_closure(&arg)
    || !cond_closures::arg_ends_with_closure(&arg) {
        return Err(ParseError::MissingClosuresError);
    }
    Ok(vec![])
}

mod cond_closures {
    static LOWER_BOUNDS_CHARS: [&str; 2] = ["[", "("];
    static UPPER_BOUNDS_CHARS: [&str; 2] = ["]", ")"];

    pub fn arg_starts_with_closure(arg: &String) -> bool {
        LOWER_BOUNDS_CHARS.iter()
            .map(|closure| arg.starts_with(closure))
            .fold(false, |acc, elem| acc || elem)
    }

    pub fn arg_ends_with_closure(arg: &String) -> bool {
        UPPER_BOUNDS_CHARS.iter()
            .map(|closure| arg.ends_with(closure))
            .fold(false, |acc, elem| acc || elem)
    }
}

#[cfg(test)]
mod parse_tests {
    use command::address_condition::parse::*;

    #[test]
    fn empty_string_returns_err() {
        let actual_result:   ParseResult = parse_arg(&String::from(""));
        let expected_result: ParseResult = Err(ParseError::ArgEmpty);
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
    use command::address_condition::parse::cond_closures::*;

    #[test]
    fn test_arg_starts_with_closure() {
        let test_cases: Vec<(String, bool)> = vec![
            (String::from("[1]"), true),
            (String::from("(1)"), true),
            (String::from("1"), false),
            (String::from("]1["), false),
            (String::from(")1("), false),
        ];
        let run_test = |test: &(String, bool)| {
            let &(ref arg, ref expected_result) = test;
            let actual_result = arg_starts_with_closure(arg);
            assert_eq!(actual_result, *expected_result,
                "Test failed for input: '{}'", arg);
        };
        test_cases.iter().for_each(|test| run_test(test));
    }

    #[test]
    fn test_arg_ends_with_closure() {
        let test_cases: Vec<(String, bool)> = vec![
            (String::from("[1]"), true),
            (String::from("(1)"), true),
            (String::from("1"), false),
            (String::from("]1["), false),
            (String::from(")1("), false),
        ];
        let run_test = |test: &(String, bool)| {
            let &(ref arg, ref expected_result) = test;
            let actual_result = arg_ends_with_closure(arg);
            assert_eq!(actual_result, *expected_result,
                "Test failed for input: '{}'", arg);
        };
        test_cases.iter().for_each(|test| run_test(test));
    }
}
