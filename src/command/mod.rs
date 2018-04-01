pub mod address_condition;
use self::address_condition::{
    Address,
    AddressCondition,
    OneAddressCondition,
    LineNumber,
    // LineRange,
    // RangeBounds,
};

pub struct Command {
    cond: Option<Box<AddressCondition>>,
}

impl Command {
    pub fn new(line:Address) -> Self {
        Self {
            cond: Some(Box::new(LineNumber::new(line))),
        }
    }

    pub fn should_run(&self, curr_line:Address) -> bool {
        match self.cond {
            Some(ref cond) => cond.applies(curr_line),
            None           => true,
        }
    }
}

#[cfg(test)]
mod comm_address_condition_tests {
    use command::*;

    #[test]
    fn line_number_condition_works() {
        let addrs: Vec<Address> = (0..10).collect();
        let comm_line: Address = 5;
        let comm = Command::new(comm_line);
        let expected_results: Vec<bool> =
            addrs.iter()
            .map(|addr: &Address| *addr == comm_line)
            .collect();
        let actual_results: Vec<bool> =
            addrs.iter()
            .map(|addr: &Address| comm.should_run(*addr))
            .collect();
        assert_eq!(actual_results, expected_results);
    }

    #[test]
    fn empty_condition_always_runs() {
        let addrs: Vec<Address> = (0..10).collect();
        let comm = Command::new(0);
        for curr_addr in addrs.iter() {
            assert_eq!(comm.should_run(*curr_addr), true);
        }
    }
}
