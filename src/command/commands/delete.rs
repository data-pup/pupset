use command::{Address, Command, Line};
use command::address_condition::Condition;

#[cfg(test)]
mod delete_tests {
    use command::{Address, Command, Line};
    use command::address_condition::{
        Condition,
        LineNumber,
    };

    fn create_lines_vec(input: &Vec<&'static str>) -> Vec<Line> {
        input.iter()
            .enumerate()
            .map(|(i, s)| Line {
                addr: i as Address,
                contents: s.to_string(),
            })
            .collect()
    }

    // fn init_test_cases() -> Vec<DeleteTestCase> {
    //     vec![
    //         DeleteTestCase {
    //             address:         Some(0),
    //             input:           vec!["hello", "world"],
    //             expected_output: vec!["",      "world"],
    //             test_desc:       "Deleting first line works.",
    //         },
    //         DeleteTestCase {
    //             address:         Some(2),
    //             input:           vec!["hello", "world"],
    //             expected_output: vec!["hello", "world"],
    //             test_desc:       "Address out of bounds deletes nothing.",
    //         },
    //         DeleteTestCase {
    //             address:         None,
    //             input:           vec!["hello", "world"],
    //             expected_output: vec!["",      ""     ],
    //             test_desc:       "No address always applies.",
    //         },
    //     ]
    // }

    struct DeleteTestCase {
        input:           Vec<&'static str>,
        address:         Option<Address>,
        expected_output: Vec<&'static str>,
        test_desc:       &'static str,
    }
}
