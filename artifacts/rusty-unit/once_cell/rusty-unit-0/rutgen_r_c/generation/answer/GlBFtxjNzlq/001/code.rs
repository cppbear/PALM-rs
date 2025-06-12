// Answer 0

#[test]
fn test_get_mut_empty_cell() {
    let mut cell: OnceCell<u32> = OnceCell::new();
    assert_eq!(cell.get_mut(), None);
}

#[test]
fn test_get_mut_with_value() {
    let mut cell: OnceCell<u32> = OnceCell::with_value(42);
    let value: Option<&mut u32> = cell.get_mut();
    assert!(value.is_some());
    if let Some(v) = value {
        *v += 1;
    }
    assert_eq!(cell.get_mut(), Some(&mut 43));
}

#[test]
fn test_get_mut_after_set() {
    let mut cell: OnceCell<u32> = OnceCell::new();
    cell.set(10).unwrap();
    let value: Option<&mut u32> = cell.get_mut();
    assert!(value.is_some());
    if let Some(v) = value {
        *v += 5;
    }
    assert_eq!(cell.get_mut(), Some(&mut 15));
}

#[test]
#[should_panic]
fn test_get_mut_after_take() {
    let mut cell: OnceCell<u32> = OnceCell::with_value(100);
    cell.take();
    cell.get_mut(); // This should panic because the cell is now empty.
}

#[test]
fn test_get_mut_after_recreation() {
    let mut cell: OnceCell<u32> = OnceCell::new();
    cell.set(99).unwrap();
    let mut_value: Option<&mut u32> = cell.get_mut().unwrap();
    assert!(mut_value.is_some());
    *mut_value += 1; // Modify the value.
    cell = OnceCell::new(); // Recreate the cell.
    assert_eq!(cell.get_mut(), None); // Ensure it's empty.
}

