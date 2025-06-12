// Answer 0

#[derive(Default)]
struct Entry {
    index: std::cell::Cell<usize>,
}

impl Entry {
    pub fn new(index: usize) -> Self {
        Entry {
            index: std::cell::Cell::new(index),
        }
    }

    pub fn index(&self) -> usize {
        *self.index.get()
    }
}

#[test]
fn test_entry_index() {
    let entry = Entry::new(5);
    assert_eq!(entry.index(), 5);
}

#[test]
fn test_entry_index_zero() {
    let entry = Entry::new(0);
    assert_eq!(entry.index(), 0);
}

#[test]
fn test_entry_index_large_value() {
    let entry = Entry::new(usize::MAX);
    assert_eq!(entry.index(), usize::MAX);
}

