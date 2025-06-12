// Answer 0

#[test]
fn test_into_body_with_integer() {
    struct TestRequest {
        body: i32,
    }
    
    let request = Request::new(10);
    let body = request.into_body();
    assert_eq!(body, 10);
}

#[test]
fn test_into_body_with_string() {
    struct TestRequest {
        body: String,
    }
    
    let request = Request::new(String::from("test"));
    let body = request.into_body();
    assert_eq!(body, "test");
}

#[test]
fn test_into_body_with_empty_vec() {
    struct TestRequest {
        body: Vec<u8>,
    }
    
    let request = Request::new(vec![]);
    let body = request.into_body();
    assert_eq!(body.len(), 0);
}

#[test]
fn test_into_body_with_non_empty_vec() {
    struct TestRequest {
        body: Vec<u8>,
    }
    
    let request = Request::new(vec![1, 2, 3]);
    let body = request.into_body();
    assert_eq!(body, vec![1, 2, 3]);
}

