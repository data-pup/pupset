use command::{Command, Line};
use command::address_condition::{Address, Condition};

struct Delete {
    cond: Option<Box<Condition>>,
}

impl Command for Delete {
    // fn new(args: Vec<String>) -> Self; // TODO

    fn should_run(&self, curr_line: Address) -> bool {
        match self.cond {
            Some(ref cond) => cond.applies(curr_line),
            None => true,
        }
    }

    fn run(&self, line: Line) -> Line {
        let Line { contents, addr } = line;
        match self.should_run(addr) {
            true => Line { contents: String::new(), addr },
            false => Line { contents, addr },
        }
    }
}

#[cfg(test)]
mod delete_tests {
    use command::{Command, Line};
    use command::commands::delete::Delete;
    use command::address_condition::{
        Address,
        Condition,
        LineNumber,
        OneAddressCondition,
    };

    #[test]
    fn simple_test() {
        let input = Line { contents: String::from("hello"), addr: 0 };
        let comm = Delete {
            cond: Some(Box::new(LineNumber::new(0))),
        };
        let output = comm.run(input);
        let actual_s = output.contents;
        let expected_s = String::from("");
        assert_eq!(actual_s, expected_s);
    }

    // fn init_test_cases() {
    //     unimplemented!();
    // }
}
