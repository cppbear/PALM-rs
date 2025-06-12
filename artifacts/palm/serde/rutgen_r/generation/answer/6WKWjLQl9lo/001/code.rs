// Answer 0

#[test]
fn test_size_hint_returns_none() {
    struct EmptyIterator;

    impl Iterator for EmptyIterator {
        type Item = ();

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let iter = EmptyIterator;
    let result = iter.size_hint();
    assert_eq!(result, None);
}

