// Answer 0

#[test]
fn test_request_new_with_string() {
    let request = Request::new("hello world".to_string());
}

#[test]
fn test_request_new_with_str() {
    let request = Request::new("hello world");
}

#[test]
fn test_request_new_with_empty_string() {
    let request = Request::new("".to_string());
}

#[test]
fn test_request_new_with_vector() {
    let data = vec![1, 2, 3];
    let request = Request::new(data);
}

#[test]
fn test_request_new_with_integer() {
    let request = Request::new(42);
}

#[test]
fn test_request_new_with_float() {
    let request = Request::new(3.14);
}

#[test]
fn test_request_new_with_boolean() {
    let request = Request::new(true);
}

#[test]
fn test_request_new_with_unit_type() {
    let request = Request::new(());
}

