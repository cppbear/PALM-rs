// Answer 0

#[derive(Debug)]
struct Char(char);

#[derive(Debug)]
struct Range(char, char);

struct Matcher {
    ranges: Vec<Range>,
}

impl Matcher {
    pub fn new(ranges: Vec<Range>) -> Self {
        Matcher { ranges }
    }
    
    pub fn matches(&self, c: Char) -> bool {
        for r in self.ranges.iter().take(4) {
            if c.0 < r.0 {
                return false;
            }
            if c.0 <= r.1 {
                return true;
            }
        }
        self.ranges.binary_search_by(|r| {
            if r.1 < c.0 {
                std::cmp::Ordering::Less
            } else if r.0 > c.0 {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }).is_ok()
    }
}

#[test]
fn test_matches_within_range() {
    let matcher = Matcher::new(vec![Range('a', 'c'), Range('e', 'g')]);
    assert!(matcher.matches(Char('a')));
    assert!(matcher.matches(Char('b')));
    assert!(matcher.matches(Char('c')));
}

#[test]
fn test_matches_outside_range() {
    let matcher = Matcher::new(vec![Range('a', 'c'), Range('e', 'g')]);
    assert!(!matcher.matches(Char('d')));
    assert!(!matcher.matches(Char('z')));
}

#[test]
fn test_matches_boundary_condition() {
    let matcher = Matcher::new(vec![Range('a', 'c'), Range('e', 'g')]);
    assert!(matcher.matches(Char('c')));
    assert!(matcher.matches(Char('e')));
}

#[test]
fn test_matches_multiple_ranges() {
    let matcher = Matcher::new(vec![Range('a', 'c'), Range('e', 'g'), Range('i', 'k')]);
    assert!(matcher.matches(Char('i')));
    assert!(!matcher.matches(Char('h')));
}

