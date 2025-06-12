// Answer 0

#[test]
fn test_status_success() {
    struct ResponseBuilder {
        status: Option<u16>,
        body: Option<()>,
    }

    impl ResponseBuilder {
        fn builder() -> Self {
            Self { status: None, body: None }
        }

        fn status(mut self, status: u16) -> Self {
            self.status = Some(status);
            self
        }

        fn body(mut self, body: ()) -> Result<Self, ()> {
            self.body = Some(body);
            Ok(self)
        }

        fn unwrap(self) -> Self {
            self
        }
    }

    let response = ResponseBuilder::builder()
        .status(200)
        .body(())
        .unwrap();
    
    assert_eq!(response.status, Some(200));
}

#[test]
#[should_panic]
fn test_status_invalid_conversion() {
    struct ResponseBuilder {
        status: Option<u16>,
        body: Option<()>,
    }

    impl ResponseBuilder {
        fn builder() -> Self {
            Self { status: None, body: None }
        }

        fn status(mut self, status: u16) -> Self {
            self.status = Some(status);
            self
        }

        fn body(mut self, body: ()) -> Result<Self, ()> {
            self.body = Some(body);
            Ok(self)
        }

        fn unwrap(self) -> Self {
            self
        }
    }

    let response = ResponseBuilder::builder()
        .status(999) // Assuming 999 is invalid for the purposes of this test
        .body(())
        .unwrap();
}

#[test]
fn test_status_default_value() {
    struct ResponseBuilder {
        status: Option<u16>,
        body: Option<()>,
    }

    impl ResponseBuilder {
        fn builder() -> Self {
            Self { status: None, body: None }
        }

        fn body(mut self, body: ()) -> Result<Self, ()> {
            self.body = Some(body);
            Ok(self)
        }

        fn unwrap(self) -> Self {
            self
        }
    }

    let response = ResponseBuilder::builder()
        .body(())
        .unwrap();
    
    assert_eq!(response.status, None);
}

#[test]
fn test_status_boundary_value() {
    struct ResponseBuilder {
        status: Option<u16>,
        body: Option<()>,
    }

    impl ResponseBuilder {
        fn builder() -> Self {
            Self { status: None, body: None }
        }

        fn status(mut self, status: u16) -> Self {
            self.status = Some(status);
            self
        }

        fn body(mut self, body: ()) -> Result<Self, ()> {
            self.body = Some(body);
            Ok(self)
        }

        fn unwrap(self) -> Self {
            self
        }
    }

    let response = ResponseBuilder::builder()
        .status(100) // Testing with the lower boundary value of HTTP status codes.
        .body(())
        .unwrap();
    
    assert_eq!(response.status, Some(100));
}

