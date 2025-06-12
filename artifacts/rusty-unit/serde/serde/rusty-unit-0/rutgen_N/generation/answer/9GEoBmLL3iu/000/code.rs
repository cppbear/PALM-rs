// Answer 0

#[derive(Debug)]
struct Iterable {
    iter: std::ops::Range<usize>,
}

impl Iterable {
    fn new(start: usize, end: usize) -> Self {
        Iterable {
            iter: start..end,
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.iter.len())
    }
}

#[test]
fn test_size_hint_non_empty() {
    let iterable = Iterable::new(0, 5);
    assert_eq!(iterable.size_hint(), Some(5));
}

#[test]
fn test_size_hint_empty() {
    let iterable = Iterable::new(5, 5);
    assert_eq!(iterable.size_hint(), Some(0));
}

#[test]
fn test_size_hint_large_range() {
    let iterable = Iterable::new(0, usize::MAX);
    assert_eq!(iterable.size_hint(), Some(usize::MAX as usize)); // Note: This is for illustrative purposes.
}

