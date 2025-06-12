// Answer 0

#[derive(Debug)]
enum Inst {
    Match(u32),
    Other,
}

impl Inst {
    pub fn is_match(&self) -> bool {
        match *self {
            Inst::Match(_) => true,
            _ => false,
        }
    }
}

#[test]
fn test_is_match_with_match_inst() {
    let instruction = Inst::Match(42);
    assert!(instruction.is_match());
}

#[test]
fn test_is_match_with_other_inst() {
    let instruction = Inst::Other;
    assert!(!instruction.is_match());
}

