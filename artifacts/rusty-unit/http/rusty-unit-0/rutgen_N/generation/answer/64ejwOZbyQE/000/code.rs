// Answer 0

#[test]
fn test_extension_with_string() {
    use std::any::Any;
    use http::{Response, Builder};

    struct MyResponse;

    impl MyResponse {
        fn builder() -> Builder {
            Builder::default()
        }
    }

    let response = MyResponse::builder()
        .extension("My Extension".to_string())
        .body(())
        .unwrap();

    assert_eq!(response.extensions().get::<String>(), Some(&"My Extension".to_string()));
}

#[test]
fn test_extension_with_integer() {
    use std::any::Any;
    use http::{Response, Builder};

    struct MyResponse;

    impl MyResponse {
        fn builder() -> Builder {
            Builder::default()
        }
    }

    let response = MyResponse::builder()
        .extension(42)
        .body(())
        .unwrap();

    assert_eq!(response.extensions().get::<i32>(), Some(&42));
}

#[test]
fn test_extension_with_custom_type() {
    use std::any::Any;
    use http::{Response, Builder};

    struct MyResponse;

    impl MyResponse {
        fn builder() -> Builder {
            Builder::default()
        }
    }

    #[derive(Clone)]
    struct MyType;

    let response = MyResponse::builder()
        .extension(MyType)
        .body(())
        .unwrap();

    assert_eq!(response.extensions().get::<MyType>(), Some(&MyType));
}

#[test]
#[should_panic]
fn test_extension_with_unconvertible_type() {
    use std::any::Any;
    use http::{Response, Builder};

    struct MyResponse;

    impl MyResponse {
        fn builder() -> Builder {
            Builder::default()
        }
    }

    let response = MyResponse::builder()
        .extension("Not an integer".to_string())
        .body(())
        .unwrap();

    // Attempt to get a different type should cause a panic
    let _ = response.extensions().get::<i32>();
}

