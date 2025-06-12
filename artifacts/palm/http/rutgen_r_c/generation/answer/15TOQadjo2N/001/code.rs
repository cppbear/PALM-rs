// Answer 0

#[test]
fn test_get_or_insert_new_value() {
    let mut ext = Extensions::new();
    *ext.get_or_insert(42i32) += 10;
    assert_eq!(*ext.get::<i32>().unwrap(), 52);
}

#[test]
fn test_get_or_insert_existing_value() {
    let mut ext = Extensions::new();
    *ext.get_or_insert(100i32) += 5;
    *ext.get_or_insert(100i32) += 10; // should not create a new entry
    assert_eq!(*ext.get::<i32>().unwrap(), 115);
}

#[test]
fn test_get_or_insert_default() {
    let mut ext = Extensions::new();
    *ext.get_or_insert_with(|| String::from("hello")) += " world";
    assert_eq!(*ext.get::<String>().unwrap(), "hello world");
}

#[test]
fn test_get_or_insert_with_custom_value() {
    let mut ext = Extensions::new();
    let value = vec![1, 2, 3];
    ext.get_or_insert(value.clone());
    assert_eq!(*ext.get::<Vec<i32>>().unwrap(), value);
}

#[test]
#[should_panic]
fn test_get_or_insert_with_panic_on_type_mismatch() {
    let mut ext = Extensions::new();
    ext.get_or_insert(10i32);
    // Trying to access a different type should panic
    let _: &str = ext.get::<str>().unwrap();
}

