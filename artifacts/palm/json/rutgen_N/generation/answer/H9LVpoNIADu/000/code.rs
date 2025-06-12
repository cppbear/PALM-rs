// Answer 0

#[derive(Debug)]
struct Discardable {
    ch: Option<char>,
}

impl Discardable {
    fn new(ch: Option<char>) -> Self {
        Discardable { ch }
    }

    fn discard(&mut self) {
        self.ch = None;
    }
}

#[test]
fn test_discard_some() {
    let mut discardable = Discardable::new(Some('a'));
    discardable.discard();
    assert_eq!(discardable.ch, None);
}

#[test]
fn test_discard_none() {
    let mut discardable = Discardable::new(None);
    discardable.discard();
    assert_eq!(discardable.ch, None);
}

