pub mod address_condition;
use self::address_condition::{
    // AddressCondition,
    // OneAddressCondition,
    LineNumber,
    // LineRange,
    // RangeBounds,
};

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
