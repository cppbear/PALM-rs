// Answer 0

#[test]
fn test_into_body_with_valid_integer() {
    struct Request {
        body: i32,
    }

    impl Request {
        pub fn new(body: i32) -> Self {
            Self { body }
        }

        pub fn into_body(self) -> i32 {
            self.body
        }
    }

    let request = Request::new(10);
    let body = request.into_body();
    assert_eq!(body, 10);
}

#[test]
fn test_into_body_with_zero() {
    struct Request {
        body: i32,
    }

    impl Request {
        pub fn new(body: i32) -> Self {
            Self { body }
        }

        pub fn into_body(self) -> i32 {
            self.body
        }
    }

    let request = Request::new(0);
    let body = request.into_body();
    assert_eq!(body, 0);
}

#[test]
fn test_into_body_with_negative_integer() {
    struct Request {
        body: i32,
    }

    impl Request {
        pub fn new(body: i32) -> Self {
            Self { body }
        }

        pub fn into_body(self) -> i32 {
            self.body
        }
    }

    let request = Request::new(-10);
    let body = request.into_body();
    assert_eq!(body, -10);
}

