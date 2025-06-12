// Answer 0

#[derive(Debug)]
struct ClassUnicodeRange {
    start: u32,
    end: u32,
}

struct UnicodeSet {
    set: Vec<ClassUnicodeRange>,
}

impl UnicodeSet {
    pub fn new() -> Self {
        UnicodeSet { set: Vec::new() }
    }
    
    pub fn push(&mut self, range: ClassUnicodeRange) {
        self.set.push(range);
    }
}

#[test]
fn test_push_valid_range() {
    let mut unicode_set = UnicodeSet::new();
    let range = ClassUnicodeRange { start: 0x0061, end: 0x007A }; // 'a' to 'z'
    unicode_set.push(range);
    assert_eq!(unicode_set.set.len(), 1);
}

#[test]
fn test_push_multiple_ranges() {
    let mut unicode_set = UnicodeSet::new();
    let range1 = ClassUnicodeRange { start: 0x0030, end: 0x0039 }; // '0' to '9'
    let range2 = ClassUnicodeRange { start: 0x00A0, end: 0x00FF }; // Non-ASCII characters
    unicode_set.push(range1);
    unicode_set.push(range2);
    assert_eq!(unicode_set.set.len(), 2);
}

#[test]
fn test_push_boundary_range() {
    let mut unicode_set = UnicodeSet::new();
    let range = ClassUnicodeRange { start: 0x0000, end: 0x10FFFF }; // Full Unicode range
    unicode_set.push(range);
    assert_eq!(unicode_set.set.len(), 1);
}

#[should_panic]
fn test_push_invalid_range() {
    let mut unicode_set = UnicodeSet::new();
    // Assuming we have a check that would panic on invalid ranges such as `start > end`
    let range = ClassUnicodeRange { start: 0x007A, end: 0x0061 }; // Invalid range
    unicode_set.push(range);
}

