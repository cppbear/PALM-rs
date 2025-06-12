// Answer 0

#[test]
fn test_serialize_str() {
    let value = "test string";
    let result = serialize_str(value);
    assert_eq!(result.unwrap(), "test string");
}

#[test]
fn test_serialize_empty_str() {
    let value = "";
    let result = serialize_str(value);
    assert_eq!(result.unwrap(), "");
}

#[test]
#[should_panic]
fn test_serialize_str_panic() {
    let value: &str = std::ptr::null();
    let _ = serialize_str(value); // This should panic 
}

