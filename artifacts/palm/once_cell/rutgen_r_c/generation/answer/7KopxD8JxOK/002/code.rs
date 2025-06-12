// Answer 0

#[test]
fn test_once_cell_debug_uninitialized() {
    struct TestOnceCell<T>(OnceCell<T>);

    impl<T: fmt::Debug> fmt::Debug for TestOnceCell<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.0.fmt(f)
        }
    }

    let once_cell: OnceCell<u32> = OnceCell::new();
    let test_struct = TestOnceCell(once_cell);
    
    let result = format!("{:?}", test_struct);
    assert_eq!(result, "OnceCell(Uninit)");
}

#[test]
fn test_once_cell_debug_initialized() {
    struct TestOnceCell<T>(OnceCell<T>);

    impl<T: fmt::Debug> fmt::Debug for TestOnceCell<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.0.fmt(f)
        }
    }

    let mut once_cell: OnceCell<u32> = OnceCell::new();
    let _ = once_cell.set(42).unwrap(); // Initialize the OnceCell
    let test_struct = TestOnceCell(once_cell);

    let result = format!("{:?}", test_struct);
    assert_eq!(result, "OnceCell(42)");
}

