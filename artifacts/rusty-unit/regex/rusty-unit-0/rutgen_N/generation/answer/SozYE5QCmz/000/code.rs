// Answer 0

#[derive(Default)]
struct Regex {
    only_utf8: bool,
}

impl Regex {
    pub fn only_utf8(&self) -> bool {
        self.only_utf8
    }
}

#[test]
fn test_only_utf8_true() {
    let regex = Regex { only_utf8: true };
    assert!(regex.only_utf8());
}

#[test]
fn test_only_utf8_false() {
    let regex = Regex { only_utf8: false };
    assert!(!regex.only_utf8());
}

