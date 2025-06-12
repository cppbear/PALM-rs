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
fn test_matches_with_empty_ranges() {
    let matcher = Matcher { ranges: vec![] };
    assert!(!matcher.matches('a'));
}

#[test]
fn test_matches_with_non_matching_character() {
    let matcher = Matcher { ranges: vec![Range('d', 'f')] };
    assert!(!matcher.matches('a'));
}

#[test]
fn test_matches_with_matching_character_in_first_range() {
    let matcher = Matcher { ranges: vec![Range('a', 'c')] };
    assert!(matcher.matches('b'));
}

#[test]
fn test_matches_with_matching_character_in_second_range() {
    let matcher = Matcher { ranges: vec![Range('g', 'j'), Range('k', 'm')] };
    assert!(matcher.matches('l'));
}

#[test]
fn test_matches_character_outside_all_ranges() {
    let matcher = Matcher { ranges: vec![Range('p', 's')] };
    assert!(!matcher.matches('a'));
}

#[test]
fn test_matches_with_exact_match_on_boundary() {
    let matcher = Matcher { ranges: vec![Range('a', 'c'), Range('d', 'f')] };
    assert!(matcher.matches('c'));
}

#[test]
fn test_matches_with_four_ranges() {
    let matcher = Matcher { ranges: vec![Range('a', 'b'), Range('c', 'd'), Range('e', 'f'), Range('g', 'h')] };
    assert!(matcher.matches('a'));
    assert!(matcher.matches('d'));
    assert!(matcher.matches('g'));
}

#[test]
fn test_matches_with_multiple_ranges_and_outside_character() {
    let matcher = Matcher { ranges: vec![Range('x', 'z'), Range('a', 'c')] };
    assert!(!matcher.matches('m'));
    assert!(matcher.matches('z'));
}

