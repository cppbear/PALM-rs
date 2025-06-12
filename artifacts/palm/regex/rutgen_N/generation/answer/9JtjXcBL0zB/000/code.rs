// Answer 0

#[derive(Debug)]
struct TestStruct<'r>(&'r str);

impl<'r> TestStruct<'r> {
    fn no_expansion(&mut self) -> Option<Cow<'r, [u8]>> {
        Some(Cow::Borrowed(self.0.as_bytes()))
    }
}

#[test]
fn test_no_expansion() {
    let mut test_instance = TestStruct("test");
    let result = test_instance.no_expansion();
    
    assert_eq!(result, Some(Cow::Borrowed(b"test")));
}

#[test]
fn test_no_expansion_empty() {
    let mut test_instance = TestStruct("");
    let result = test_instance.no_expansion();
    
    assert_eq!(result, Some(Cow::Borrowed(b"")));
}

