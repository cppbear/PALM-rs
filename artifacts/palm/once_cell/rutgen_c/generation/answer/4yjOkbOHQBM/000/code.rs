// Answer 0

#[test]
fn test_try_insert_success() {
    struct TestCell {
        cell: OnceCell<i32>,
    }

    let cell = TestCell {
        cell: OnceCell::new(),
    };

    assert!(cell.cell.get().is_none());
    assert_eq!(cell.cell.try_insert(42), Ok(&42));
    assert_eq!(cell.cell.get(), Some(&42));
}

#[test]
fn test_try_insert_failure() {
    struct TestCell {
        cell: OnceCell<i32>,
    }

    let mut cell = TestCell {
        cell: OnceCell::new(),
    };

    assert!(cell.cell.get().is_none());
    assert_eq!(cell.cell.try_insert(100), Ok(&100));
    assert_eq!(cell.cell.try_insert(200), Err((&100, 200)));
    assert_eq!(cell.cell.get(), Some(&100));
}

#[test]
fn test_try_insert_overwrite() {
    struct TestCell {
        cell: OnceCell<String>,
    }

    let mut cell = TestCell {
        cell: OnceCell::new(),
    };

    assert!(cell.cell.get().is_none());
    assert_eq!(cell.cell.try_insert("Initial".to_string()), Ok(&"Initial".to_string()));
    assert_eq!(cell.cell.try_insert("Overwrite".to_string()), Err((&"Initial".to_string(), "Overwrite".to_string())));
}

