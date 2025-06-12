// Answer 0

#[derive(Default)]
struct Sparse {
    size: usize,
}

impl Sparse {
    pub fn clear(&mut self) {
        self.size = 0;
    }
}

#[test]
fn test_clear() {
    let mut sparse = Sparse::default();
    sparse.size = 5; // Set an initial size
    sparse.clear();
    assert_eq!(sparse.size, 0);
}

#[test]
fn test_clear_already_cleared() {
    let mut sparse = Sparse::default();
    sparse.clear(); // Call clear when size is already 0
    assert_eq!(sparse.size, 0);
}

