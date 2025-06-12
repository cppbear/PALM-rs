// Answer 0

#[test]
fn test_new_lazy_with_valid_function() {
    let lazy_value = new(|| 42);
    assert!(lazy_value.cell.get().is_none());
    assert!(lazy_value.init.get().is_some());
}

#[test]
fn test_new_lazy_with_empty_function() {
    let lazy_value = new(|| {});
    assert!(lazy_value.cell.get().is_none());
    assert!(lazy_value.init.get().is_some());
}

#[test]
#[should_panic]
fn test_new_lazy_with_panic_function() {
    let _ = new(|| panic!("This function panics"));
}

