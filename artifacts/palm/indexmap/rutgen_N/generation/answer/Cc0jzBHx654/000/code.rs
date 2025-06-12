// Answer 0

#[derive(Debug)]
struct Slice<T> {
    data: Vec<T>,
}

impl<T> Slice<T> {
    fn from_slice(slice: &[T]) -> &Self {
        // Unsafe conversion; for the purpose of the example.
        unsafe { &*(slice as *const [T] as *const Self) }
    }
}

struct TestIterator<T> {
    data: Vec<T>,
    index: usize,
}

impl<T> TestIterator<T> {
    fn new(data: Vec<T>) -> Self {
        Self { data, index: 0 }
    }

    pub fn as_slice(&self) -> &Slice<T> {
        Slice::from_slice(&self.data[self.index..])
    }
}

#[test]
fn test_as_slice_non_empty() {
    let iterator = TestIterator::new(vec![1, 2, 3]);
    let slice = iterator.as_slice();
    assert_eq!(slice.data.len(), 3);
    assert_eq!(slice.data[0], 1);
    assert_eq!(slice.data[1], 2);
    assert_eq!(slice.data[2], 3);
}

#[test]
fn test_as_slice_empty() {
    let iterator = TestIterator::new(vec![]);
    let slice = iterator.as_slice();
    assert_eq!(slice.data.len(), 0);
}

#[test]
fn test_as_slice_boundary() {
    let iterator = TestIterator::new(vec![1]);
    let slice = iterator.as_slice();
    assert_eq!(slice.data.len(), 1);
    assert_eq!(slice.data[0], 1);
}

