// Answer 0

#[test]
fn test_into_body_with_basic_integer() {
    struct Response {
        body: i32,
    }

    impl Response {
        pub fn new(body: i32) -> Self {
            Response { body }
        }

        pub fn into_body(self) -> i32 {
            self.body
        }
    }

    let response = Response::new(10);
    let body = response.into_body();
    assert_eq!(body, 10);
}

#[test]
fn test_into_body_with_negative_integer() {
    struct Response {
        body: i32,
    }

    impl Response {
        pub fn new(body: i32) -> Self {
            Response { body }
        }

        pub fn into_body(self) -> i32 {
            self.body
        }
    }

    let response = Response::new(-5);
    let body = response.into_body();
    assert_eq!(body, -5);
}

#[test]
fn test_into_body_with_large_integer() {
    struct Response {
        body: i32,
    }

    impl Response {
        pub fn new(body: i32) -> Self {
            Response { body }
        }

        pub fn into_body(self) -> i32 {
            self.body
        }
    }

    let response = Response::new(i32::MAX);
    let body = response.into_body();
    assert_eq!(body, i32::MAX);
}

