// Answer 0

#[test]
fn test_get_mut_existing_value() {
    let mut ext = Extensions::new();
    ext.insert(String::from("Hello"));
    let value = ext.get_mut::<String>().unwrap();
    value.push_str(" World");

    assert_eq!(ext.get::<String>().unwrap(), "Hello World");
}

#[test]
fn test_get_mut_non_existent_value() {
    let mut ext = Extensions::new();
    assert_eq!(ext.get_mut::<String>(), None);
}

#[test]
fn test_get_mut_with_empty_extensions() {
    let mut ext = Extensions::new();
    ext.clear();
    assert_eq!(ext.get_mut::<i32>(), None);
}

#[test]
fn test_get_mut_after_removal() {
    let mut ext = Extensions::new();
    ext.insert(String::from("Hello"));
    ext.remove::<String>(); // Remove the existing value
    assert_eq!(ext.get_mut::<String>(), None);
}

