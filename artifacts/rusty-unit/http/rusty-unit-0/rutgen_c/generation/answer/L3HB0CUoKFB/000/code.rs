// Answer 0

#[test]
fn test_into_body_with_integer() {
    let response = Response::new(10);
    let body = response.into_body();
    assert_eq!(body, 10);
}

#[test]
fn test_into_body_with_string() {
    let response = Response::new(String::from("Hello, world!"));
    let body = response.into_body();
    assert_eq!(body, "Hello, world!");
}

#[test]
fn test_into_body_with_empty_vector() {
    let response = Response::new(vec![]);
    let body = response.into_body();
    assert_eq!(body, vec![]);
}

#[test]
fn test_into_body_with_custom_struct() {
    struct CustomStruct {
        value: i32,
    }
    let response = Response::new(CustomStruct { value: 5 });
    let body = response.into_body();
    assert_eq!(body.value, 5);
}

