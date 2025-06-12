// Answer 0

#[derive(Debug)]
struct Slice<K, V> {
    data: Vec<(K, V)>,
}

impl<K, V> Slice<K, V> {
    fn from_mut_slice(slice: &mut [(K, V)]) -> &mut Self {
        unsafe {
            &mut *(slice as *mut [(K, V)] as *mut Slice<K, V>)
        }
    }
}

struct Iter<K, V> {
    iter: Vec<(K, V)>,
}

impl<K, V> Iter<K, V> {
    fn into_slice(self) -> &'static mut Slice<K, V> {
        Slice::from_mut_slice(self.iter.as_mut_slice())
    }
}

#[test]
fn test_into_slice() {
    let iter = Iter {
        iter: vec![(1, "a"), (2, "b"), (3, "c")],
    };

    let slice = iter.into_slice();
    assert_eq!(slice.data.len(), 3);
    assert_eq!(slice.data[0], (1, "a"));
}

#[test]
fn test_into_slice_empty() {
    let iter = Iter { iter: vec![] };

    let slice = iter.into_slice();
    assert_eq!(slice.data.len(), 0);
}

