// Answer 0

#[test]
fn test_get_or_insert_with_creates_new_value() {
    let mut ext = Extensions::new();
    let value = ext.get_or_insert_with(|| 42);
    assert_eq!(*value, 42);
}

#[test]
fn test_get_or_insert_with_retrieves_existing_value() {
    let mut ext = Extensions::new();
    *ext.get_or_insert_with(|| 42) += 1; // Set initial value to 42, then increment
    let value = ext.get_or_insert_with(|| 100); // Should return the already incremented value
    assert_eq!(*value, 43);
}

#[test]
fn test_get_or_insert_with_multiple_types() {
    let mut ext = Extensions::new();
    let int_value = ext.get_or_insert_with(|| 42);
    let str_value = ext.get_or_insert_with(|| String::from("Hello"));

    assert_eq!(*int_value, 42);
    assert_eq!(*str_value, "Hello");
}

#[test]
fn test_get_or_insert_with_default_value() {
    let mut ext = Extensions::new();
    let value: &mut Vec<i32> = ext.get_or_insert_with(|| Vec::new());
    value.push(10);
    
    assert_eq!(value.len(), 1);
    assert_eq!(value[0], 10);
}

