// Answer 0

#[test]
fn test_no_expansion_with_borrowed_data() {
    use std::borrow::Cow;

    struct TestStruct<'a>(&'a [u8]);

    impl TestStruct<'_> {
        fn no_expansion(&mut self) -> Option<Cow<[u8]>> {
            Some(Cow::Borrowed(self.0))
        }
    }

    let data: &[u8] = &[1, 2, 3, 4, 5];
    let mut test_struct = TestStruct(data);
    let result = test_struct.no_expansion();

    assert_eq!(result, Some(Cow::Borrowed(data)));
}

#[test]
fn test_no_expansion_with_empty_slice() {
    use std::borrow::Cow;

    struct TestStruct<'a>(&'a [u8]);

    impl TestStruct<'_> {
        fn no_expansion(&mut self) -> Option<Cow<[u8]>> {
            Some(Cow::Borrowed(self.0))
        }
    }

    let data: &[u8] = &[];
    let mut test_struct = TestStruct(data);
    let result = test_struct.no_expansion();

    assert_eq!(result, Some(Cow::Borrowed(data)));
}

#[test]
fn test_no_expansion_with_large_slice() {
    use std::borrow::Cow;

    struct TestStruct<'a>(&'a [u8]);

    impl TestStruct<'_> {
        fn no_expansion(&mut self) -> Option<Cow<[u8]>> {
            Some(Cow::Borrowed(self.0))
        }
    }

    let data: &[u8] = &[0; 1000]; // Large slice with 1000 zeroes
    let mut test_struct = TestStruct(data);
    let result = test_struct.no_expansion();

    assert_eq!(result, Some(Cow::Borrowed(data)));
}

