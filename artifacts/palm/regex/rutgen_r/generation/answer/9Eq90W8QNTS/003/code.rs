// Answer 0

#[derive(Debug)]
struct Range(char, char);

struct Matcher {
    ranges: Vec<Range>,
}

impl Matcher {
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

#[test]
fn test_matches_within_range() {
    let matcher = Matcher {
        ranges: vec![Range('a', 'd'), Range('e', 'h'), Range('i', 'l'), Range('m', 'p')],
    };
    assert_eq!(matcher.matches('b'), true); // c <= r.1
}

#[test]
fn test_matches_boundary_condition_first() {
    let matcher = Matcher {
        ranges: vec![Range('a', 'd'), Range('e', 'h'), Range('i', 'l'), Range('m', 'p')],
    };
    assert_eq!(matcher.matches('a'), true); // c == r.0
}

#[test]
fn test_matches_boundary_condition_last() {
    let matcher = Matcher {
        ranges: vec![Range('a', 'd'), Range('e', 'h'), Range('i', 'l'), Range('m', 'p')],
    };
    assert_eq!(matcher.matches('d'), true); // c == r.1
}

#[test]
fn test_matches_out_of_range() {
    let matcher = Matcher {
        ranges: vec![Range('a', 'd'), Range('e', 'h'), Range('i', 'l'), Range('m', 'p')],
    };
    assert_eq!(matcher.matches('z'), false); // c outside of all ranges
}

#[test]
fn test_matches_no_ranges() {
    let matcher = Matcher { ranges: vec![] };
    assert_eq!(matcher.matches('a'), false); // no ranges to match
}

