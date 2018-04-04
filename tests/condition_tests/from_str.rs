// use {
//     Address,
//     AddressCondition,
//     AddressConditionParseError,
//     Values,
// };

// type ParseResult = Result<AddressCondition, AddressConditionParseError>;

// struct LineNumberTest {
//     input:           &'static str,
//     addr:            Address,
//     expected_result: ParseResult,
//     desc:            &'static str,
// }

// const LINE_NUMBER_TESTS: &[LineNumberTest] = &[
//     LineNumberTest {
//         input: "[0]",
//         addr: 0,
//         expected_result: Ok(AddressCondition {
//             vals: Values::LineNumber(0),
//         }),
//         desc: "Single digit (0) inclusively enclosed",
//     }
// ];

// #[test]
// fn run_line_number_tests() {
//     // let input = "[0]";
//     // let comm = input.parse::<AddressCondition>().unwrap();
//     // assert_eq!(comm.applies(0), true);
//     // assert_eq!(comm.applies(1), false);
//     assert_eq!(1, 2, "Unimplemented!");
// }