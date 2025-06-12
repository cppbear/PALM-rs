// Answer 0

#[derive(Debug)]
struct Character {
    c: char,
}

struct CharacterClassRange {
    start: Character,
    end: Character,
}

impl CharacterClassRange {
    pub fn is_valid(&self) -> bool {
        self.start.c <= self.end.c
    }
}

#[test]
fn test_valid_range() {
    let range = CharacterClassRange {
        start: Character { c: 'a' },
        end: Character { c: 'z' },
    };
    assert!(range.is_valid());
}

#[test]
fn test_invalid_range() {
    let range = CharacterClassRange {
        start: Character { c: 'z' },
        end: Character { c: 'a' },
    };
    assert!(!range.is_valid());
}

#[test]
fn test_equal_range() {
    let range = CharacterClassRange {
        start: Character { c: 'a' },
        end: Character { c: 'a' },
    };
    assert!(range.is_valid());
}

