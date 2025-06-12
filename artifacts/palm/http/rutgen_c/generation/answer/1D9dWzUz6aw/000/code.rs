// Answer 0

#[test]
fn test_body() {
    struct DummyBody {
        content: String,
    }
    
    let request = Request::new(DummyBody {
        content: String::from("Test content"),
    });

    assert_eq!(request.body().content, "Test content");
}

#[test]
fn test_empty_body() {
    let request: Request<String> = Request::new(String::new());
    assert!(request.body().is_empty());
}

