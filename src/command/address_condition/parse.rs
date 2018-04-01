use command::address_condition::{
    Address
};

pub type AddressList = Vec<Address>;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    ArgEmpty,
    InvalidLowerBounds,
    InvalidUpperBounds,
    InvalidAddressNumber,
}

pub type ParseResult = Result<AddressList, ParseError>;

static LOWER_BOUNDS_CHARS: [char; 2] = ['[', '('];
static UPPER_BOUNDS_CHARS: [char; 2] = [']', ')'];

pub fn parse_arg(arg: String) -> ParseResult {
    if arg.is_empty() { return Err(ParseError::ArgEmpty); }
    Ok(vec![])
}

#[cfg(test)]
mod parse_tests {
    use command::address_condition::parse::*;
    // use command::address_condition::Address;

    #[test]
    fn empty_string_returns_err() {
        let actual_result:   ParseResult = parse_arg(String::from(""));
        let expected_result: ParseResult = Err(ParseError::ArgEmpty);
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
