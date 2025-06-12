// Answer 0

#[test]
fn test_no_expansion_some() {
    use std::borrow::Cow;

    struct TestStruct(Cow<str>);

    impl TestStruct {
        fn no_expansion(&mut self) -> Option<Cow<str>> {
            Some(self.0.clone())
        }
    }

    let mut test_instance = TestStruct(Cow::Borrowed("test"));
    assert_eq!(test_instance.no_expansion(), Some(Cow::Borrowed("test")));
}

#[test]
fn test_no_expansion_none() {
    use std::borrow::Cow;

    struct TestStruct(Cow<str>);

    impl TestStruct {
        fn no_expansion(&mut self) -> Option<Cow<str>> {
            None
        }
    }

    let mut test_instance = TestStruct(Cow::Borrowed("test"));
    assert_eq!(test_instance.no_expansion(), None);
}

