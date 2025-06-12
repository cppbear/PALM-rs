// Answer 0

#[derive(Debug)]
struct TestMap {
    entries: Vec<i32>,
}

impl TestMap {
    fn from_slice(slice: &[i32]) -> Self {
        TestMap {
            entries: slice.to_vec(),
        }
    }

    fn split_at(&self, index: usize) -> (&Self, &Self) {
        let (first, second) = self.entries.split_at(index);
        (Self::from_slice(first), Self::from_slice(second))
    }
}

#[test]
fn test_split_at_valid_index() {
    let map = TestMap::from_slice(&[1, 2, 3, 4, 5]);
    let (first, second) = map.split_at(2);

    assert_eq!(first.entries, vec![1, 2]);
    assert_eq!(second.entries, vec![3, 4, 5]);
}

#[test]
fn test_split_at_zero_index() {
    let map = TestMap::from_slice(&[1, 2, 3, 4, 5]);
    let (first, second) = map.split_at(0);

    assert_eq!(first.entries, vec![]);
    assert_eq!(second.entries, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_split_at_full_length_index() {
    let map = TestMap::from_slice(&[1, 2, 3, 4, 5]);
    let (first, second) = map.split_at(5);

    assert_eq!(first.entries, vec![1, 2, 3, 4, 5]);
    assert_eq!(second.entries, vec![]);
}

#[should_panic]
#[test]
fn test_split_at_panic_on_out_of_bounds() {
    let map = TestMap::from_slice(&[1, 2, 3, 4, 5]);
    let _ = map.split_at(6);
}

