pub mod address_condition;
mod parse;

use self::address_condition::{
    Address,
    Condition,
    OneAddressCondition,
    LineNumber,
    parse_arg,
    // LineRange,
    // RangeBounds,
};

pub struct Command {
    cond: Option<Box<Condition>>,
}

impl Command {
    pub fn new(addr:Option<Address>) -> Self {
        Self {
            cond: match addr {
                Some(a) => Some(Box::new(LineNumber::new(a))),
                None    => None,
            }
        }
    }

    /// Check whether a command should run for the current line.
    pub fn should_run(&self, curr_line:Address) -> bool {
        match self.cond {
            Some(ref cond) => cond.applies(curr_line),
            None           => true,
        }
    }

    /// Create a command using a vector of argument strings.
    pub fn from_args(args: Vec<String>) -> Self {
        unimplemented!();
    }
}

#[cfg(test)]
mod comm_address_condition_tests {
    use command::*;

    #[test]
    fn line_number_condition_works() {
        let addrs: Vec<Address> = (0..10).collect();
        let comm_line: Address = 5;
        let comm = Command::new(Some(comm_line));
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
        let comm = Command::new(None);
        for curr_addr in addrs.iter() {
            assert_eq!(comm.should_run(*curr_addr), true);
        }
    }
}
