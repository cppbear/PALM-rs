// Answer 0

#[derive(Debug)]
struct Slice<T> {
    data: Vec<T>,
}

impl<T> Slice<T> {
    fn from_slice(slice: &[T]) -> &Self {
        // Safety: assuming slice is valid for the lifetime of the returned reference
        unsafe { &*(slice as *const [T] as *const Self) }
    }
}

struct Iter<T> {
    iter: Vec<T>,
}

impl<T> Iter<T> {
    pub fn as_slice(&self) -> &Slice<T> {
        Slice::from_slice(self.iter.as_slice())
    }
}

#[test]
fn test_as_slice_non_empty() {
    let iter = Iter {
        iter: vec![1, 2, 3, 4],
    };
    let slice = iter.as_slice();
    assert_eq!(slice.data, vec![1, 2, 3, 4]);
}

#[test]
fn test_as_slice_empty() {
    let iter: Iter<i32> = Iter { iter: vec![] };
    let slice = iter.as_slice();
    assert_eq!(slice.data, vec![]);
}

