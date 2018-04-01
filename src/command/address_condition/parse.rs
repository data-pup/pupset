use command::address_condition::{
    Address
};

pub type AddressList = Vec<Address>;

static LOWER_BOUNDS_CHARS: [char; 2] = ['[', '('];

pub fn parse_arg(arg: String) -> Result<AddressList, String> {
    arg.split(",");

    unimplemented!();
}

#[cfg(test)]
mod parse_tests {
    use command::address_condition::parse::*;

    #[test]
    fn it_works() {
        assert_eq!(1, 1);
    }
}
