// Answer 0

#[test]
fn test_matches_false_with_c_less_than_r0() {
    struct RangeMatcher {
        ranges: Vec<(char, char)>,
    }

    impl RangeMatcher {
        pub fn matches(&self, c: char) -> bool {
            for r in self.ranges.iter().take(4) {
                if c < r.0 {
                    return false;
                }
                if c <= r.1 {
                    return true;
                }
            }
            self.ranges.binary_search_by(|r| {
                if r.1 < c {
                    std::cmp::Ordering::Less
                } else if r.0 > c {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            }).is_ok()
        }
    }

    let matcher = RangeMatcher {
        ranges: vec![('f', 'j'), ('k', 'n'), ('o', 'q'), ('r', 't')],
    };

    let result = matcher.matches('a');
    assert_eq!(result, false);
}

