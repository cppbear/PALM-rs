// Answer 0

#[test]
fn test_get_or_insert_with_value_inserted() {
    let mut ext = Extensions::new();
    let value = ext.get_or_insert_with(|| 10i32);
    *value += 5;
    assert_eq!(*ext.get::<i32>().unwrap(), 15);
}

#[test]
fn test_get_or_insert_with_second_insertion() {
    let mut ext = Extensions::new();
    let value_first = ext.get_or_insert_with(|| 20i32);
    let value_second = ext.get_or_insert_with(|| 30i32);
    *value_first += 10;
    assert_eq!(*value_second, 30);
    assert_eq!(*ext.get::<i32>().unwrap(), 30); 
}

#[test]
fn test_get_or_insert_with_string() {
    let mut ext = Extensions::new();
    let value = ext.get_or_insert_with(|| "Hello".to_string());
    value.push_str(", World!");
    assert_eq!(*ext.get::<String>().unwrap(), "Hello, World!");
}

#[test]
#[should_panic]
fn test_get_or_insert_with_unexpected_type() {
    let mut ext = Extensions::new();
    let _value: &i32 = ext.get_or_insert_with(|| 42i32);
    let _unexpected: &str = ext.get::<str>().unwrap(); // This should panic
}

#[test]
fn test_get_or_insert_with_default_value() {
    let mut ext = Extensions::new();
    let value = ext.get_or_insert_with(|| 0u32);
    *value += 1;
    assert_eq!(*ext.get::<u32>().unwrap(), 1);
}

#[test]
fn test_get_or_insert_with_multiple_types() {
    let mut ext = Extensions::new();
    let value_i32 = ext.get_or_insert_with(|| 234i32);
    let value_f64 = ext.get_or_insert_with(|| 56.78f64);
    *value_i32 += 1;
    *value_f64 += 1.22;
    assert_eq!(*ext.get::<i32>().unwrap(), 235);
    assert_eq!(*ext.get::<f64>().unwrap(), 57.0);
}

#[test]
fn test_get_or_insert_with_panic_on_or_insert() {
    let mut ext = Extensions::new();
    ext.get_or_insert_with(|| {
        panic!("Testing panic during insertion");
    });
}

#[test]
fn test_get_or_insert_mutable_access() {
    let mut ext = Extensions::new();
    let value = ext.get_or_insert_with(|| 100i64);
    *value += 50;
    assert_eq!(*ext.get::<i64>().unwrap(), 150);
}

