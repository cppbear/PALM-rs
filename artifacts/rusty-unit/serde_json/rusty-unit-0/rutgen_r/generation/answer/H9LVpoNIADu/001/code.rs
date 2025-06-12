// Answer 0

#[derive(Debug)]
struct TestStruct {
    ch: Option<char>,
}

impl TestStruct {
    fn new(ch: Option<char>) -> Self {
        TestStruct { ch }
    }

    fn discard(&mut self) {
        self.ch = None;
    }
}

#[test]
fn test_discard_with_some() {
    let mut test_instance = TestStruct::new(Some('a'));
    test_instance.discard();
    assert_eq!(test_instance.ch, None);
}

#[test]
fn test_discard_with_none() {
    let mut test_instance = TestStruct::new(None);
    test_instance.discard();
    assert_eq!(test_instance.ch, None);
}

#[test]
#[should_panic]
fn test_discard_on_unwrapped_none() {
    let mut test_instance = TestStruct::new(None);
    // This doesn't panic directly but is designed to showcase the handling of None.
    test_instance.discard();
    assert_eq!(test_instance.ch.unwrap(), 'a'); // this will panic as we're trying to unwrap None
}

