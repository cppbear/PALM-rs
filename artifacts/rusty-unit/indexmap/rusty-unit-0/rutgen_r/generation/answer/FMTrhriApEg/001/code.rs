// Answer 0

#[derive(Debug)]
struct Entry {
    index: usize,
}

impl Entry {
    pub fn new(index: usize) -> Self {
        Entry { index }
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

#[test]
fn test_index_zero() {
    let entry = Entry::new(0);
    assert_eq!(entry.index(), 0);
}

#[test]
fn test_index_positive() {
    let entry = Entry::new(42);
    assert_eq!(entry.index(), 42);
}

#[test]
fn test_index_large_number() {
    let entry = Entry::new(usize::MAX);
    assert_eq!(entry.index(), usize::MAX);
}

