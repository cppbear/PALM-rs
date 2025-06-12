// Answer 0

#[test]
fn test_once_cell_with_value() {
    struct TestStruct {
        value: i32,
    }
    
    let cell = OnceCell::with_value(TestStruct { value: 42 });
    assert_eq!(cell.get().map(|s| s.value), Some(42));
}

#[test]
fn test_once_cell_with_different_types() {
    // Using a different struct to test generics
    struct AnotherStruct {
        name: String,
    }
    
    let cell = OnceCell::with_value(AnotherStruct { name: String::from("Test") });
    assert_eq!(cell.get().map(|s| &s.name), Some(&String::from("Test")));
}

#[test]
fn test_once_cell_with_value_empty() {
    struct EmptyStruct;
    
    let cell: OnceCell<EmptyStruct> = OnceCell::new();
    assert_eq!(cell.get(), None);
}

