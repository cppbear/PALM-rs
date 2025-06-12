// Answer 0

#[test]
fn test_get_mut_when_cell_is_empty() {
    let mut cell: OnceCell<u32> = OnceCell::new();
    assert!(cell.get_mut().is_none());
}

#[test]
fn test_get_mut_after_setting_value() {
    let mut cell: OnceCell<u32> = OnceCell::new();
    cell.set(42).unwrap();
    let value = cell.get_mut().unwrap();
    *value = 43;  // Modify the value through mutable reference
    assert_eq!(cell.get(), Some(&43));
}

#[test]
fn test_get_mut_multiple_access() {
    let mut cell: OnceCell<u32> = OnceCell::new();
    cell.set(99).unwrap();
    let value1 = cell.get_mut().unwrap();
    *value1 = 100; // First mutation
    let value2 = cell.get_mut().unwrap();
    *value2 += 1;  // Second mutation
    assert_eq!(cell.get(), Some(&101));
}

#[test]
fn test_get_mut_after_take() {
    let mut cell: OnceCell<u32> = OnceCell::new();
    cell.set(50).unwrap();
    assert_eq!(cell.take(), Some(50));
    assert!(cell.get_mut().is_none());  // Should be None since we've taken the value
}

#[test]
fn test_get_mut_with_uninitialized_cell_after_set() {
    let mut cell: OnceCell<String> = OnceCell::new();
    cell.set("Hello".to_string()).unwrap();
    let value = cell.get_mut().unwrap();
    value.push_str(", World!");  // Mutate the string
    assert_eq!(cell.get(), Some(&"Hello, World!".to_string()));
}

