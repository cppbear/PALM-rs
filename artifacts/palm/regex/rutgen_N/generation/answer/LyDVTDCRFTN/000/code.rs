// Answer 0

#[derive(Debug)]
struct CharRange {
    start: char,
    end: char,
}

impl CharRange {
    pub fn new(start: char, end: char) -> Self {
        CharRange { start, end }
    }
    
    pub fn start(&self) -> char {
        self.start
    }
}

#[test]
fn test_char_range_start() {
    let range = CharRange::new('a', 'z');
    assert_eq!(range.start(), 'a');
}

#[test]
fn test_char_range_boundary_start() {
    let range = CharRange::new('0', '9');
    assert_eq!(range.start(), '0');
}

#[test]
fn test_char_range_single_character() {
    let range = CharRange::new('x', 'x');
    assert_eq!(range.start(), 'x');
}

