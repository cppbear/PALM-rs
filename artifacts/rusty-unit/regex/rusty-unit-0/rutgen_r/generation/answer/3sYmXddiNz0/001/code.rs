// Answer 0


use std::borrow::Cow;

struct TestStruct(Cow<str>);

impl TestStruct {
    fn no_expansion(&mut self) -> Option<Cow<str>> {
        if self.0 == Cow::Borrowed("") {
            None
        } else {
            Some(Cow::Borrowed(self.0.as_ref()))
        }
    }
}

#[test]
fn test_no_expansion_with_empty_string() {
    let mut test_struct = TestStruct(Cow::Borrowed(""));
    assert_eq!(test_struct.no_expansion(), None);
}

#[test]
fn test_no_expansion_with_non_empty_string() {
    let mut test_struct = TestStruct(Cow::Borrowed("Hello, world!"));
    assert_eq!(test_struct.no_expansion(), Some(Cow::Borrowed("Hello, world!")));
}

#[test]
fn test_no_expansion_with_unicode_string() {
    let mut test_struct = TestStruct(Cow::Borrowed("こんにちは")); // "Hello" in Japanese
    assert_eq!(test_struct.no_expansion(), Some(Cow::Borrowed("こんにちは")));
}

#[test]
fn test_no_expansion_with_whitespace_string() {
    let mut test_struct = TestStruct(Cow::Borrowed("   ")); // string with spaces
    assert_eq!(test_struct.no_expansion(), Some(Cow::Borrowed("   ")));
}


