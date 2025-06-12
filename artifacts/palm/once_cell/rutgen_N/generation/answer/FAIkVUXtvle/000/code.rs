// Answer 0

#[derive(Debug)]
struct OnceNonZeroUsize {
    value: Option<usize>,
}

impl OnceNonZeroUsize {
    pub const fn new() -> Self {
        Self { value: None }
    }
}

struct Cell {
    inner: OnceNonZeroUsize,
}

impl Cell {
    pub const fn new() -> Self {
        Self { inner: OnceNonZeroUsize::new() }
    }
}

#[test]
fn test_new_cell() {
    let cell = Cell::new();
    assert_eq!(cell.inner.value, None);
}

