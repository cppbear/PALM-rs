// Answer 0

#[test]
fn test_new_cell() {
    use std::sync::atomic::AtomicUsize;

    struct Cell {
        inner: AtomicUsize,
    }

    impl Cell {
        pub const fn new() -> Self {
            Self { inner: AtomicUsize::new(0) }
        }
    }

    let cell = Cell::new();
    assert_eq!(cell.inner.load(std::sync::atomic::Ordering::SeqCst), 0);
}

#[test]
fn test_new_cell_non_zero_initialization() {
    use std::sync::atomic::AtomicUsize;

    struct Cell {
        inner: AtomicUsize,
    }

    impl Cell {
        pub const fn new() -> Self {
            Self { inner: AtomicUsize::new(0) }
        }
    }

    let cell = Cell::new();
    assert_eq!(cell.inner.load(std::sync::atomic::Ordering::SeqCst), 0);
}

