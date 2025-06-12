// Answer 0

#[test]
fn test_get_mut_on_non_empty_cell() {
    use once_cell::unsync::OnceCell;

    let mut cell: OnceCell<u32> = OnceCell::new();
    cell.set(92).unwrap();
    
    // Modify the value through mutable access
    {
        let value_mut = cell.get_mut().unwrap();
        *value_mut = 93;
    }

    assert_eq!(cell.get(), Some(&93));
}

#[test]
fn test_get_mut_on_empty_cell() {
    use once_cell::unsync::OnceCell;

    let mut cell: OnceCell<u32> = OnceCell::new();
    
    // Attempt to get mutable reference from an empty cell
    let result = cell.get_mut();

    assert!(result.is_none());
}

#[test]
#[should_panic]
fn test_get_mut_after_setting_once() {
    use once_cell::unsync::OnceCell;

    let mut cell: OnceCell<u32> = OnceCell::new();
    cell.set(42).unwrap();

    {
        let _value_mut = cell.get_mut().unwrap();
        // After this, we shouldn't be able to set again
    }
    
    // This should panic since we're trying to access mutably again which violates the OnceCell's constraints
    let _result = cell.get_mut().unwrap();
}

