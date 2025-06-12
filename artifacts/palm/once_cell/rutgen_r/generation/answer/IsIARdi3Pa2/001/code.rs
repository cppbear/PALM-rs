// Answer 0

#[test]
fn test_new_empty_cell() {
    struct Cell {
        inner: std::sync::atomic::AtomicUsize,
    }

    impl Cell {
        pub const fn new() -> Self {
            Self { inner: std::sync::atomic::AtomicUsize::new(0) }
        }
    }

    let cell = Cell::new();
    assert_eq!(cell.inner.load(std::sync::atomic::Ordering::SeqCst), 0);
}

