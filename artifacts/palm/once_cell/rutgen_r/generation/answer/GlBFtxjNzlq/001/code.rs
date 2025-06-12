// Answer 0

#[test]
fn test_get_mut_when_cell_is_empty() {
    use once_cell::sync::OnceCell;

    let mut cell: OnceCell<u32> = OnceCell::new();
    assert_eq!(cell.get_mut(), None);
}

#[test]
fn test_get_mut_after_set() {
    use once_cell::sync::OnceCell;

    let mut cell: OnceCell<u32> = OnceCell::new();
    cell.set(42).unwrap();
    assert!(cell.get_mut().is_some());
}

#[test]
fn test_get_mut_on_non_empty_cell() {
    use once_cell::sync::OnceCell;

    let mut cell: OnceCell<u32> = OnceCell::new();
    cell.set(10).unwrap();
    let value = cell.get_mut().unwrap();
    *value += 5; // modify the value
    assert_eq!(*value, 15);
}

#[test]
fn test_get_mut_after_reset() {
    use once_cell::sync::OnceCell;

    let mut cell: OnceCell<u32> = OnceCell::new();
    cell.set(100).unwrap();
    let _ = cell.get_mut();
    cell.set(200).unwrap(); // reset the cell
    assert_eq!(cell.get_mut(), None);
}

