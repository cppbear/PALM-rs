// Answer 0

#[derive(Debug)]
struct TestIterator;

impl Iterator for TestIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

#[test]
fn test_size_hint_none() {
    let iter = TestIterator;
    assert_eq!(iter.size_hint(), (0, None));
}

