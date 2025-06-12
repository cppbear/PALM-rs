// Answer 0

#[test]
fn test_get_mut_when_empty() {
    use once_cell::sync::OnceCell;

    let mut cell: OnceCell<u32> = OnceCell::new();
    assert_eq!(cell.get_mut(), None);
}

#[test]
fn test_get_mut_when_set() {
    use once_cell::sync::OnceCell;

    let mut cell: OnceCell<u32> = OnceCell::new();
    cell.set(42).unwrap();
    let value = cell.get_mut().unwrap();
    *value += 1; // Modify the value
    assert_eq!(*value, 43);
    
    // Confirm that changing the value again through the mutable reference works
    assert_eq!(cell.get(), Some(&43));
}

#[test]
#[should_panic]
fn test_get_mut_after_set_and_reinitialize() {
    use once_cell::sync::OnceCell;

    let mut cell: OnceCell<u32> = OnceCell::new();
    cell.set(100).unwrap();
    let _value = cell.get_mut().unwrap();
    cell = OnceCell::new(); // Reinitialize the cell
    // Attempt to use get_mut after reinitialization should panic
    let _new_value = cell.get_mut().unwrap(); 
}

