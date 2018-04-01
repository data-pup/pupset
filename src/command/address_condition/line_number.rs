use command::address_condition::{
    Address,
    AddressCondition,
    OneAddressCondition,
};

pub struct LineNumber {
    n: Address,
}

impl OneAddressCondition for LineNumber {
    fn new(addr: Address) -> Self {
        LineNumber { n:addr }
    }
}

impl AddressCondition for LineNumber {
    fn applies(&self, current_line: Address) -> bool { self.n == current_line }
}

#[cfg(test)]
mod tests {
    use command::address_condition::line_number::*;

    #[test]
    fn line_number_tests() {
        let addresses: Vec<Address> =
            (0..11).collect(); // Create a sequence [0, 1, ..., 10].
        let test_conditions: Vec<LineNumber> =
            vec![0, 5, 10]         // Create a condition for lines 0, 5, and 10.
            .iter()
            .map(|n: &Address| LineNumber::new(*n))
            .collect();
        let expected_results = vec![
            //  [0,     5,     10] <- Condition results.
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
        let condition_results: Vec<Vec<bool>> = addresses
            .iter()
            .map(|curr_addr: &Address| -> Vec<bool> {
                test_conditions
                    .iter()
                    .map(|curr_cond: &LineNumber| -> bool {
                        curr_cond.applies(*curr_addr)
                    })
                    .collect()
            })
            .collect();
        let mut i: usize = 0;
        for expected in expected_results.iter() {
            let actual = &condition_results[i];
            assert_eq!(*expected, *actual, "Test failed at line: {}", i);
            i += 1;
        }
    }
}
