use command::address_condition::{
    Address
};

pub fn parse_arg(arg: String) -> Address {
    0
}

#[cfg(test)]
mod parse_tests {
    use command::address_condition::parse::*;

    #[test]
    fn it_works() {
        assert_eq!(parse_arg(String::from("0")), 0);
    }
}
