// Answer 0

#[test]
fn test_case_fold_simple_empty_ranges() {
    struct IntervalSet {
        ranges: Vec<Range>,
    }

    struct Range {
        start: char,
        end: char,
    }

    impl Range {
        fn case_fold_simple(&self, ranges: &mut Vec<Range>) {
            // Dummy implementation, for testing purposes creates the case folded range
            if self.start.is_ascii_alphabetic() && self.end.is_ascii_alphabetic() {
                let lowered = self.start.to_lowercase();
                let uppered = self.start.to_uppercase();
                ranges.push(Range { start: lowered.chars().next().unwrap(), end: uppered.chars().next().unwrap() });
            }
        }
    }

    impl IntervalSet {
        fn canonicalize(&mut self) {
            // Dummy implementation, does nothing for this test
        }

        pub fn case_fold_simple(&mut self) {
            let len = self.ranges.len();
            for i in 0..len {
                let range = &self.ranges[i];
                range.case_fold_simple(&mut self.ranges);
            }
            self.canonicalize();
        }
    }

    let mut set = IntervalSet { ranges: Vec::new() };
    set.case_fold_simple(); // This should not panic and simply do nothing

    assert!(set.ranges.is_empty()); // Ensure the ranges are still empty
}

