// Answer 0

#[derive(Debug)]
struct Entry<K> {
    key: K,
}

struct Slice<K> {
    entries: Vec<Entry<K>>,
}

impl<K> Slice<K> {
    fn new(entries: Vec<Entry<K>>) -> Self {
        Self { entries }
    }
}

impl<K> Slice<K> {
    fn index(&self, index: usize) -> &K {
        &self.entries[index].key
    }
}

#[test]
fn test_index_with_valid_index() {
    let entry1 = Entry { key: 1 };
    let entry2 = Entry { key: 2 };
    let slice = Slice::new(vec![entry1, entry2]);

    assert_eq!(*slice.index(0), 1);
    assert_eq!(*slice.index(1), 2);
}

#[test]
#[should_panic]
fn test_index_with_out_of_bounds() {
    let entry1 = Entry { key: 1 };
    let slice = Slice::new(vec![entry1]);

    // This will panic because the index is out of bounds
    let _ = slice.index(1);
}

