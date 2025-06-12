// Answer 0

#[test]
fn test_case_fold_simple() {
    struct TestSet {
        ranges: Vec<(char, char)>,
    }

    impl TestSet {
        fn case_fold_simple(&mut self) {
            // A simplistic representation of case folding, for demonstration purposes
            let mut new_ranges = Vec::new();
            for &(start, end) in &self.ranges {
                new_ranges.push((start, end)); // original range
                // Add case folded range; here we assume simple alphabetical case folding
                if start.is_ascii_alphabetic() {
                    let start_upper = start.to_ascii_uppercase();
                    let end_upper = end.to_ascii_uppercase();
                    new_ranges.push((start_upper, end_upper));
                }
            }
            self.ranges = new_ranges;
        }
    }

    let mut test_set = TestSet {
        ranges: vec![('a', 'z')],
    };
    test_set.case_fold_simple();

    assert_eq!(
        test_set.ranges,
        vec![('a', 'z'), ('A', 'Z')]
    );

    let mut test_set_mixed = TestSet {
        ranges: vec![('a', 'c'), ('1', '3')],
    };
    test_set_mixed.case_fold_simple();

    assert_eq!(
        test_set_mixed.ranges,
        vec![('a', 'c'), ('A', 'C'), ('1', '3')]
    );
}

