// Answer 0

#[derive(Debug)]
struct Slice<T> {
    data: Vec<T>,
}

impl<T> Slice<T> {
    fn from_slice(slice: &[T]) -> &Slice<T> {
        &Slice { data: slice.to_vec() }
    }
}

struct IteratorTest<T> {
    iter: Vec<T>,
    index: usize,
}

impl<T> IteratorTest<T> {
    fn new(data: Vec<T>) -> Self {
        IteratorTest { iter: data, index: 0 }
    }

    fn as_slice(&self) -> &Slice<T> {
        Slice::from_slice(&self.iter[self.index..])
    }

    fn advance(&mut self) {
        self.index += 1;
    }
}

#[test]
fn test_as_slice_empty() {
    let iter = IteratorTest::new(Vec::<i32>::new());
    let slice = iter.as_slice();
    assert_eq!(slice.data.len(), 0);
}

#[test]
fn test_as_slice_one_element() {
    let iter = IteratorTest::new(vec![42]);
    let slice = iter.as_slice();
    assert_eq!(slice.data.len(), 1);
    assert_eq!(slice.data[0], 42);
}

#[test]
fn test_as_slice_multiple_elements() {
    let iter = IteratorTest::new(vec![1, 2, 3, 4, 5]);
    let slice = iter.as_slice();
    assert_eq!(slice.data.len(), 5);
    assert_eq!(slice.data, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_as_slice_after_advance() {
    let mut iter = IteratorTest::new(vec![10, 20, 30, 40, 50]);
    iter.advance(); // Move past the first element
    let slice = iter.as_slice();
    assert_eq!(slice.data.len(), 4);
    assert_eq!(slice.data, vec![20, 30, 40, 50]);
}

#[test]
fn test_as_slice_advance_to_end() {
    let mut iter = IteratorTest::new(vec![100, 200, 300]);
    iter.advance(); // Move past the first element
    iter.advance(); // Move past the second element
    iter.advance(); // Move past the third element
    let slice = iter.as_slice();
    assert_eq!(slice.data.len(), 0);
}

