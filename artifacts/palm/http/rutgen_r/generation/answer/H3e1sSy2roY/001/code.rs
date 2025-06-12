// Answer 0

#[test]
fn test_body_mut_with_non_empty_body() {
    struct Response<T> {
        body: T,
    }

    impl<T> Default for Response<T>
    where
        T: Default,
    {
        fn default() -> Self {
            Response {
                body: T::default(),
            }
        }
    }

    let mut response: Response<String> = Response::default();
    response.body_mut().push_str("initial body");
    
    let body_ref = response.body_mut();
    body_ref.push_str(" updated body");
    
    assert_eq!(response.body, "initial body updated body");
}

#[test]
fn test_body_mut_with_empty_body() {
    struct Response<T> {
        body: T,
    }

    impl<T> Default for Response<T>
    where
        T: Default,
    {
        fn default() -> Self {
            Response {
                body: T::default(),
            }
        }
    }

    let mut response: Response<String> = Response::default();
    
    {
        let body_ref = response.body_mut();
        body_ref.push_str("body contents");
    }
    
    assert_eq!(response.body, "body contents");
}

#[test]
fn test_body_mut_without_initialization() {
    struct Response<T> {
        body: T,
    }

    impl Default for Response<String> {
        fn default() -> Self {
            Response {
                body: String::new(),
            }
        }
    }

    let mut response: Response<String> = Response::default();
    let body_ref = response.body_mut();
    
    body_ref.push_str("new body");
    
    assert!(!response.body.is_empty());
    assert_eq!(response.body, "new body");
}

