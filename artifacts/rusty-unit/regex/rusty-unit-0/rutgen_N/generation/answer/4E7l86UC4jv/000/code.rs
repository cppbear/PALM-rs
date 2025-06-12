// Answer 0

#[derive(Debug)]
struct Ranges {
    ranges: Vec<(char, char)>,
}

impl Ranges {
    fn new(ranges: Vec<(char, char)>) -> Self {
        Ranges { ranges }
    }

    pub fn num_chars(&self) -> usize {
        self.ranges.iter()
            .map(|&(s, e)| 1 + (e as u32) - (s as u32))
            .fold(0, |acc, len| acc + len) as usize
    }
}

#[test]
fn test_num_chars_empty() {
    let ranges = Ranges::new(vec![]);
    assert_eq!(ranges.num_chars(), 0);
}

#[test]
fn test_num_chars_single_range() {
    let ranges = Ranges::new(vec![('a', 'c')]);
    assert_eq!(ranges.num_chars(), 3);
}

#[test]
fn test_num_chars_multiple_ranges() {
    let ranges = Ranges::new(vec![('a', 'c'), ('e', 'g')]);
    assert_eq!(ranges.num_chars(), 6);
}

#[test]
fn test_num_chars_overlapping_ranges() {
    let ranges = Ranges::new(vec![('a', 'd'), ('b', 'f')]);
    assert_eq!(ranges.num_chars(), 6);
}

#[test]
fn test_num_chars_adjacent_ranges() {
    let ranges = Ranges::new(vec![('a', 'a'), ('b', 'b')]);
    assert_eq!(ranges.num_chars(), 2);
}

#[test]
fn test_num_chars_palindrome_ranges() {
    let ranges = Ranges::new(vec![('a', 'e'), ('e', 'z')]);
    assert_eq!(ranges.num_chars(), 26);
}

