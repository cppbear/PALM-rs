// Answer 0

#[derive(Default)]
struct Entry {
    index: std::cell::Cell<Option<usize>>,
}

impl Entry {
    pub fn new(index: Option<usize>) -> Self {
        Entry {
            index: std::cell::Cell::new(index),
        }
    }
    
    pub fn index(&self) -> usize {
        *self.index.get().expect("Index is None")
    }
}

#[test]
fn test_index_with_some_value() {
    let entry = Entry::new(Some(42));
    assert_eq!(entry.index(), 42);
}

#[test]
#[should_panic(expected = "Index is None")]
fn test_index_with_none_value() {
    let entry = Entry::new(None);
    entry.index();
}

#[test]
fn test_index_with_zero() {
    let entry = Entry::new(Some(0));
    assert_eq!(entry.index(), 0);
}

#[test]
fn test_index_with_large_value() {
    let entry = Entry::new(Some(usize::MAX));
    assert_eq!(entry.index(), usize::MAX);
}

