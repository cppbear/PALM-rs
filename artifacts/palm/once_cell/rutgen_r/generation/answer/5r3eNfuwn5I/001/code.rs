// Answer 0

#[test]
fn test_set_when_full_returns_err() {
    use once_cell::sync::OnceCell;

    struct TestCell {
        cell: OnceCell<i32>,
    }

    impl TestCell {
        fn new() -> Self {
            TestCell {
                cell: OnceCell::new(),
            }
        }

        fn set(&self, value: i32) -> Result<(), i32> {
            self.cell.set(value)
        }
    }

    let test_cell = TestCell::new();
    
    // First, populate the cell with a value
    assert_eq!(test_cell.set(100), Ok(()));
    
    // Now, try setting a new value; it should fail
    assert_eq!(test_cell.set(200), Err(200));
}

