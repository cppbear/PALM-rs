// Answer 0

#[test]
fn test_once_cell_set_empty() {
    struct TestCell {
        cell: OnceCell<i32>,
    }

    // Initialize the OnceCell
    let mut test_cell = TestCell {
        cell: OnceCell::new(),
    };

    // Validate the cell is empty initially
    assert!(test_cell.cell.get().is_none());

    // Set a value and expect Ok(())
    assert_eq!(test_cell.cell.set(42), Ok(()));
    
    // Validate the cell is now set with the value
    assert_eq!(test_cell.cell.get(), Some(&42));
}

#[test]
fn test_once_cell_set_non_empty() {
    struct TestCell {
        cell: OnceCell<i32>,
    }

    // Initialize the OnceCell
    let mut test_cell = TestCell {
        cell: OnceCell::with_value(100),
    };

    // Validate the cell is not empty and holds the initial value
    assert_eq!(test_cell.cell.get(), Some(&100));

    // Try setting a new value and expect Err(value)
    assert_eq!(test_cell.cell.set(200), Err(200));
    
    // Validate that the cell still holds the original value
    assert_eq!(test_cell.cell.get(), Some(&100));
}

