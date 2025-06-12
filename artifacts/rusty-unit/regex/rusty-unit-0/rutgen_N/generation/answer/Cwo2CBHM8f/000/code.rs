// Answer 0

#[test]
fn test_no_expansion_return_none() {
    use std::borrow::Cow;

    struct TestStruct;

    impl TestStruct {
        fn no_expansion<'r>(&'r mut self) -> Option<Cow<'r, [u8]>> {
            None
        }
    }

    let mut instance = TestStruct;
    assert_eq!(instance.no_expansion(), None);
}

