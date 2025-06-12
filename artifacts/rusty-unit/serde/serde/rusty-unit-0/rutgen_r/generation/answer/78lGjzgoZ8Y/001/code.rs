// Answer 0

#[derive(Debug)]
struct TestIterator {
    iter: Vec<i32>,
}

impl TestIterator {
    fn new(iter: Vec<i32>) -> Self {
        TestIterator { iter }
    }

    fn size_hint(&self) -> Option<usize> {
        self.iter.len().checked_sub(1)
    }
}

#[test]
fn test_size_hint_non_empty() {
    let iterator = TestIterator::new(vec![1, 2, 3, 4]);
    assert_eq!(iterator.size_hint(), Some(3));
}

#[test]
fn test_size_hint_empty() {
    let iterator = TestIterator::new(vec![]);
    assert_eq!(iterator.size_hint(), Some(0));
}

#[test]
fn test_size_hint_single_element() {
    let iterator = TestIterator::new(vec![42]);
    assert_eq!(iterator.size_hint(), Some(0));
}

#[test]
fn test_size_hint_large_iter() {
    let iterator = TestIterator::new((1..1000).collect());
    assert_eq!(iterator.size_hint(), Some(999));
}

#[test]
#[should_panic]
fn test_size_hint_overflow() {
    let iterator = TestIterator::new(vec![i32::MAX]);
    assert_eq!(iterator.size_hint(), Some(0)); // This can't panic, but if we were checking an overflow, it would be a concern
}

