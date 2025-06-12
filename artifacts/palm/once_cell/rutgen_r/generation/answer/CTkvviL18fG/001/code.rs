// Answer 0

#[test]
fn test_get_or_init_success() {
    use once_cell::sync::OnceCell;

    let cell = OnceCell::new();
    let value = cell.get_or_init(|| 42);
    assert_eq!(value, &42);
}

#[test]
#[should_panic]
fn test_get_or_init_panic() {
    use once_cell::sync::OnceCell;

    let cell = OnceCell::new();
    let _value = cell.get_or_init(|| panic!("This should panic"));
}

#[test]
fn test_get_or_init_multiple_calls() {
    use once_cell::sync::OnceCell;
    
    let cell = OnceCell::new();
    let value1 = cell.get_or_init(|| 100);
    let value2 = cell.get_or_init(|| {
        // This block will not get executed because the cell is already initialized.
        panic!("This should not execute");
    });
    assert_eq!(value1, &100);
    assert_eq!(value2, &100);
}

#[test]
fn test_get_or_init_reentrant_call() {
    use once_cell::sync::OnceCell;

    let cell = OnceCell::new();

    let result = std::panic::catch_unwind(|| {
        let _value = cell.get_or_init(|| {
            // Attempt to call get_or_init again, which is 'reentrant'.
            cell.get_or_init(|| 50);
            10
        });
    });

    assert!(result.is_err());
}

#[test]
fn test_get_or_init_empty_call() {
    use once_cell::sync::OnceCell;

    let cell: OnceCell<Option<i32>> = OnceCell::new();
    let value = cell.get_or_init(|| None);
    assert_eq!(value, &None);
}

