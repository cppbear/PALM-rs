// Answer 0

#[test]
fn test_get_or_try_init_with_initialized_cell() {
    let cell = OnceCell::with_value(42);
    let result = cell.get_or_try_init(|| Ok(100));
}

#[test]
fn test_get_or_try_init_with_empty_cell_fails() {
    let cell = OnceCell::new();
    let result = cell.get_or_try_init(|| Err(()));
}

#[test]
fn test_get_or_try_init_with_successful_initialization() {
    let cell = OnceCell::new();
    let value = cell.get_or_try_init(|| Ok(99));
}

#[test]
#[should_panic]
fn test_get_or_try_init_with_panic_in_initialization() {
    let cell = OnceCell::new();
    let _result = cell.get_or_try_init(|| panic!("Initialization panic"));
}

