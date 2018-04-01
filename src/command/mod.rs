pub mod address_condition;
use self::address_condition::AddressCondition;
use self::address_condition::OneAddressCondition;
use self::address_condition::line_number::LineNumber;

pub struct Command {
    cond: LineNumber,
}

#[cfg(test)]
mod tests {
    use command::*;

    #[test]
    fn it_works() {
        Command { cond: LineNumber::new(0_32) };
    }
}
