use command::address_condition::{
    Address
};

pub type ParseResult = Result<AddressList, ParseError>;
pub type AddressList = Vec<Address>;
#[derive(Debug, PartialEq)]
pub enum ParseError {
    ArgEmpty,
    ArgTooShort,
    InvalidLowerBounds,
    InvalidUpperBounds,
    InvalidAddressNumber,
}

static LOWER_BOUNDS_CHARS: [char; 2] = ['[', '('];
static UPPER_BOUNDS_CHARS: [char; 2] = [']', ')'];
static MINIMUM_LENGTH: usize = 3;

pub fn parse_arg(arg: String) -> ParseResult {
    if arg.is_empty() { return Err(ParseError::ArgEmpty); }
    if arg.len() < MINIMUM_LENGTH { return Err(ParseError::ArgTooShort); }
    Ok(vec![])
}

#[cfg(test)]
mod parse_tests {
    use command::address_condition::parse::*;

    #[test]
    fn empty_string_returns_err() {
        let actual_result:   ParseResult = parse_arg(String::from(""));
        let expected_result: ParseResult = Err(ParseError::ArgEmpty);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn empty_closure_returns_err() {
        let actual_result:   ParseResult = parse_arg(String::from("[]"));
        let expected_result: ParseResult = Err(ParseError::ArgTooShort);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn missing_closure_returns_err() {
        let actual_result:   ParseResult = parse_arg(String::from("1"));
        let expected_result: ParseResult = Err(ParseError::ArgTooShort);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn parse_line_number() {
        let arg = String::from("[10]");
        let actual_result:   ParseResult = parse_arg(arg);
        let expected_result: ParseResult = Ok(vec![10]);
        assert_eq!(actual_result, expected_result);
    }
}
