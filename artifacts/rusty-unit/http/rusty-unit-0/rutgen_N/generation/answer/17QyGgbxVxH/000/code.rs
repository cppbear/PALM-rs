// Answer 0

#[test]
fn test_from_parts() {
    // Define necessary data structures directly here.
    struct Parts {
        status: StatusCode,
    }

    struct Response<T> {
        head: Parts,
        body: T,
    }

    impl<T> Response<T> {
        fn from_parts(parts: Parts, body: T) -> Response<T> {
            Response { head: parts, body }
        }
        
        fn status(&self) -> StatusCode {
            self.head.status
        }
        
        fn body(&self) -> &T {
            &self.body
        }
    }

    #[derive(PartialEq, Debug)]
    enum StatusCode {
        OK,
        BAD_REQUEST,
    }

    // Initialize Parts and body
    let parts = Parts { status: StatusCode::OK };
    let body = "hello world";
    
    // Create response from parts and body
    let response = Response::from_parts(parts, body);
    
    // Check the assertions
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(*response.body(), "hello world");
}

#[test]
fn test_from_parts_with_bad_request() {
    struct Parts {
        status: StatusCode,
    }

    struct Response<T> {
        head: Parts,
        body: T,
    }

    impl<T> Response<T> {
        fn from_parts(parts: Parts, body: T) -> Response<T> {
            Response { head: parts, body }
        }
        
        fn status(&self) -> StatusCode {
            self.head.status
        }
        
        fn body(&self) -> &T {
            &self.body
        }
    }

    #[derive(PartialEq, Debug)]
    enum StatusCode {
        OK,
        BAD_REQUEST,
    }

    // Initialize Parts and body
    let parts = Parts { status: StatusCode::BAD_REQUEST };
    let body = "hello world";
    
    // Create response from parts and body
    let response = Response::from_parts(parts, body);
    
    // Check the assertions
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(*response.body(), "hello world");
}

