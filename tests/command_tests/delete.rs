use command::{
    Command,
    CommandType::Delete,
};

struct DeleteTestCase {
    input:           Vec<&'static str>,
    address:         Option<Address>,
    expected_output: Vec<&'static str>,
    test_desc:       &'static str,
}

#[test]
fn it_works() {
    let comm = Command { comm: Delete };
}

// fn create_lines_vec(input: &Vec<&'static str>) -> Vec<Line> {
//     input.iter()
//         .enumerate()
//         .map(|(i, s)| Line {
//             addr: i as Address,
//             contents: s.to_string(),
//         })
//         .collect()
// }

// #[test]
// fn delete_works() {
//     for test_case in init_test_cases().iter() {
//         let &DeleteTestCase {
//             address, ref input, ref expected_output, test_desc,
//         } = test_case;
//         let comm = match address {
//             Some(addr) => Delete {
//                 cond: Some(Box::new(LineNumber::new(addr)))
//             },
//             None => Delete { cond: None },
//         };
//         let lines:          Vec<Line> = create_lines_vec(input);
//         let expected_lines: Vec<Line> = create_lines_vec(expected_output);
//         let actual_lines:   Vec<Line> = lines.into_iter()
//             .map(|l| comm.run(l))
//             .collect();
//         assert_eq!(actual_lines, expected_lines,
//             "Test Failed: {}", test_desc);
//     }
// }

fn init_test_cases() -> Vec<DeleteTestCase> {
    vec![
        DeleteTestCase {
            address:         Some(0),
            input:           vec!["hello", "world"],
            expected_output: vec!["",      "world"],
            test_desc:       "Deleting first line works.",
        },
        DeleteTestCase {
            address:         Some(2),
            input:           vec!["hello", "world"],
            expected_output: vec!["hello", "world"],
            test_desc:       "Address out of bounds deletes nothing.",
        },
        DeleteTestCase {
            address:         None,
            input:           vec!["hello", "world"],
            expected_output: vec!["",      ""     ],
            test_desc:       "No address always applies.",
        },
    ]
}
