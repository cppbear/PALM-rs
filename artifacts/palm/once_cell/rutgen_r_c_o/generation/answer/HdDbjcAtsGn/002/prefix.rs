// Answer 0

#[test]
fn test_get_or_init_with_small_value() {
    let cell: OnceCell<i32> = OnceCell::new();
    let value = cell.get_or_init(|| 1);
}

#[test]
fn test_get_or_init_with_large_value() {
    let cell: OnceCell<i32> = OnceCell::new();
    let value = cell.get_or_init(|| 1_000_000);
}

#[test]
fn test_get_or_init_with_zero() {
    let cell: OnceCell<i32> = OnceCell::new();
    let value = cell.get_or_init(|| 0);
}

#[test]
fn test_get_or_init_with_negative_value() {
    let cell: OnceCell<i32> = OnceCell::new();
    let value = cell.get_or_init(|| -1);
}

#[test]
fn test_get_or_init_with_boundary_values() {
    let cell: OnceCell<u32> = OnceCell::new();
    let value = cell.get_or_init(|| u32::MAX);
    let value_low = cell.get_or_init(|| u32::MIN);
}

#[test]
#[should_panic]
fn test_get_or_init_with_panic_on_initialization() {
    let cell: OnceCell<i32> = OnceCell::new();
    let _value = cell.get_or_init(|| panic!("Initialization Panic"));
}

