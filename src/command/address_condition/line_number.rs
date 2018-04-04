use command::Address;
use command::address_condition::AddressCondition;

#[derive(Debug, PartialEq)]
pub struct LineNumber {
    n: Address,
}

impl LineNumber {
    pub fn new(addr: Address) -> Self {
        LineNumber { n:addr }
    }
}

#[cfg(test)]
mod tests {
    use command::address_condition::line_number::*;

    struct TestCase {
        addresses: Vec<Address>,
        conditions: Vec<LineNumber>,
        expected_results: Vec<Vec<bool>>,
    }

    /// Create a vector of LineNumber conditions for lines 0, 5, and 10.
    fn init_test_objects() -> TestCase {
        let addresses: Vec<Address> = (0..11).collect();
        let conditions = vec![0, 5, 10]
            .iter()
            .map(|n: &Address| LineNumber::new(*n))
            .collect();
        let expected_results = vec![
            //  [0,     5,     10   ]  <- Expected condition results.
            vec![true,  false, false], // Line 0
            vec![false, false, false], // Line 1
            vec![false, false, false], // Line 2
            vec![false, false, false], // Line 3
            vec![false, false, false], // Line 4
            vec![false, true,  false], // Line 5
            vec![false, false, false], // Line 6
            vec![false, false, false], // Line 7
            vec![false, false, false], // Line 8
            vec![false, false, false], // Line 9
            vec![false, false, true ], // Line 10
        ];
        TestCase { addresses, conditions, expected_results }
    }

    // #[test]
    // fn line_number_tests() {
    //     let TestCase { addresses, conditions, expected_results } = init_test_objects();

    //     let condition_results: Vec<Vec<bool>> =
    //         addresses.iter()    // Iterate through the addresses.
    //         .map(|curr_addr: &Address| -> Vec<bool> {
    //             conditions      // For each address, check which conditions
    //                 .iter()     // apply to the current line number.
    //                 .map(|curr_cond: &LineNumber| -> bool {
    //                     curr_cond.applies(*curr_addr)
    //                 })
    //                 .collect()
    //         })
    //         .collect();

    //     let mut i: usize = 0; // Check the test results line by line.
    //     for expected in expected_results.iter() {
    //         let actual = &condition_results[i];
    //         assert_eq!(*expected, *actual, "Test failed at line: {}", i);
    //         i += 1;
    //     }
    // }
}
