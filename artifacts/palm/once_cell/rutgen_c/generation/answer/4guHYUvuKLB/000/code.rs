// Answer 0

#[test]
fn test_get_mut_returns_none_when_empty() {
    struct TestCell {
        value: OnceCell<u32>,
    }

    let mut cell = TestCell { value: OnceCell::new() };
    assert_eq!(cell.value.get_mut(), None);
}

#[test]
fn test_get_mut_returns_mut_reference() {
    struct TestCell {
        value: OnceCell<u32>,
    }

    let mut cell = TestCell { value: OnceCell::new() };
    cell.value.set(42).unwrap();

    let mut_ref = cell.value.get_mut().unwrap();
    *mut_ref = 50;

    assert_eq!(cell.value.get(), Some(&50));
}

#[test]
fn test_get_mut_fails_if_cell_set_multiple_times() {
    struct TestCell {
        value: OnceCell<u32>,
    }

    let mut cell = TestCell { value: OnceCell::new() };
    cell.value.set(10).unwrap();
    
    {
        let mut_ref = cell.value.get_mut().unwrap();
        *mut_ref = 20; // Modify the value
    }
    
    assert_eq!(cell.value.get(), Some(&20));
    
    // Further attempts to mutate should still be valid using get_mut
    let mut_ref = cell.value.get_mut().unwrap();
    *mut_ref = 30;

    assert_eq!(cell.value.get(), Some(&30));
}

