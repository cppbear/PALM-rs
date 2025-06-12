// Answer 0

#[test]
fn test_set_empty_cell() {
    struct OnceCellWrapper {
        cell: OnceCell<i32>,
    }

    let cell = OnceCellWrapper {
        cell: OnceCell::new(),
    };
    
    assert!(cell.cell.get().is_none());
    assert_eq!(cell.cell.set(42), Ok(()));
    assert_eq!(cell.cell.get(), Some(&42));
}

#[test]
fn test_set_non_empty_cell() {
    struct OnceCellWrapper {
        cell: OnceCell<i32>,
    }

    let cell = OnceCellWrapper {
        cell: OnceCell::new(),
    };

    assert_eq!(cell.cell.set(42), Ok(()));
    assert_eq!(cell.cell.set(24), Err(24));
    assert_eq!(cell.cell.get(), Some(&42));
}

#[test]
fn test_set_multiple_values() {
    struct OnceCellWrapper {
        cell: OnceCell<i32>,
    }

    let cell = OnceCellWrapper {
        cell: OnceCell::new(),
    };

    assert_eq!(cell.cell.set(1), Ok(()));
    assert_eq!(cell.cell.set(2), Err(2));
    assert_eq!(cell.cell.set(3), Err(3));
    assert_eq!(cell.cell.get(), Some(&1));
}

