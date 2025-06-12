// Answer 0

#[test]
fn test_no_expansion_returns_none() {
    struct Dummy;

    impl Dummy {
        fn no_expansion<'r>(&'r mut self) -> Option<Cow<'r, str>> {
            None
        }
    }

    let mut dummy = Dummy;
    assert_eq!(dummy.no_expansion(), None);
}

