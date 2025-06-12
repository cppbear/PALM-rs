// Answer 0

#[test]
fn test_set_with_full_cell() {
    use once_cell::unsync::OnceCell;

    struct Test {
        cell: OnceCell<i32>,
    }

    let mut test = Test {
        cell: OnceCell::new(),
    };

    // First, set a value to fill the cell
    assert_eq!(test.cell.set(42), Ok(()));
    
    // Now attempt to set another value, which should return an Err with the initial value
    assert_eq!(test.cell.set(99), Err(42));
}

#[test]
fn test_set_with_full_cell_edge_case() {
    use once_cell::unsync::OnceCell;

    struct Test {
        cell: OnceCell<i32>,
    }

    let mut test = Test {
        cell: OnceCell::new(),
    };
    
    // Set a value to fill the cell
    assert_eq!(test.cell.set(-1), Ok(()));
    
    // Attempt to set a value again; it should return an Err with the existing value
    assert_eq!(test.cell.set(0), Err(-1));
}

#[test]
fn test_set_with_full_cell_large_value() {
    use once_cell::unsync::OnceCell;

    struct Test {
        cell: OnceCell<u64>,
    }

    let mut test = Test {
        cell: OnceCell::new(),
    };

    // Set a large value in the cell
    assert_eq!(test.cell.set(1_000_000_000_000), Ok(()));
    
    // Attempt to set another large value; should return an Err with the first value
    assert_eq!(test.cell.set(999_999_999_999), Err(1_000_000_000_000));
}

