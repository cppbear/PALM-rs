// Answer 0

#[test]
fn test_body_mut() {
    struct MyResponse {
        content: String,
    }

    impl Default for MyResponse {
        fn default() -> Self {
            MyResponse {
                content: String::new(),
            }
        }
    }

    let mut response: Response<MyResponse> = Response::new(MyResponse::default());
    response.body_mut().content.push_str("hello world");
    assert!(!response.body().content.is_empty());
}

#[test]
fn test_body_mut_empty_initialization() {
    struct MyResponse {
        content: String,
    }

    impl Default for MyResponse {
        fn default() -> Self {
            MyResponse {
                content: String::new(),
            }
        }
    }

    let mut response: Response<MyResponse> = Response::new(MyResponse::default());
    assert!(response.body().content.is_empty());
    response.body_mut().content.push_str("test");
    assert_eq!(response.body().content, "test");
}

