// Answer 0

#[test]
fn test_into_body() {
    struct Response<T> {
        body: T,
    }

    impl<T> Response<T> {
        pub fn new(body: T) -> Self {
            Response { body }
        }

        pub fn into_body(self) -> T {
            self.body
        }
    }

    let response = Response::new(10);
    let body = response.into_body();
    assert_eq!(body, 10);
}

