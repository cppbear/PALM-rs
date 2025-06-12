// Answer 0

#[test]
fn test_lazy_new() {
    use super::Lazy;

    fn init_function() -> i32 {
        42
    }

    let lazy_value: Lazy<i32, fn() -> i32> = Lazy::new(init_function);
    // Should succeed in initializing the lazy value without invoking the init function yet.
    assert!(lazy_value.cell.get().is_none());
}

#[test]
fn test_lazy_new_function_signature() {
    use super::Lazy;

    fn init_fn() -> String {
        String::from("Hello, World!")
    }

    let lazy_string: Lazy<String, fn() -> String> = Lazy::new(init_fn);
    // Check that the cell is initialized to None
    assert!(lazy_string.cell.get().is_none());
}

