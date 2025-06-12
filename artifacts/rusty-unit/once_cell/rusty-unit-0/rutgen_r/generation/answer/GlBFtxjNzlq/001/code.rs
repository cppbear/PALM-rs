// Answer 0

#[test]
fn test_get_mut_when_cell_is_empty() {
    use once_cell::sync::OnceCell;

    let mut cell: OnceCell<u32> = OnceCell::new();
    let result = cell.get_mut();
    assert!(result.is_none());
}

#[test]
fn test_get_mut_when_cell_is_set() {
    use once_cell::sync::OnceCell;

    let mut cell: OnceCell<u32> = OnceCell::new();
    cell.set(42).unwrap();
    let result = cell.get_mut();
    assert!(result.is_some());
    assert_eq!(*result.unwrap(), 42);
}

#[test]
fn test_get_mut_cell_mutation() {
    use once_cell::sync::OnceCell;

    let mut cell: OnceCell<u32> = OnceCell::new();
    cell.set(10).unwrap();
    {
        let value = cell.get_mut().unwrap();
        *value += 5;
    }
    assert_eq!(cell.get_mut().unwrap(), &15);
}

#[test]
#[should_panic]
fn test_get_mut_after_reinitializing_cell() {
    use once_cell::sync::OnceCell;

    let mut cell: OnceCell<u32> = OnceCell::new();
    cell.set(50).unwrap();
    cell = OnceCell::new(); // Reinitializing the cell
    let _ = cell.get_mut(); // This should panic as the cell is empty now
}

