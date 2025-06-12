// Answer 0

#[test]
fn test_response_builder_creation() {
    struct Response;
    struct Builder;

    impl Builder {
        fn new() -> Self {
            Builder
        }

        fn status(self, _status: u16) -> Self {
            self
        }

        fn header(self, _name: &str, _value: &str) -> Self {
            self
        }

        fn body(self, _body: ()) -> Result<Response, &'static str> {
            Ok(Response)
        }
    }

    let response = Response::builder()
        .status(200)
        .header("X-Custom-Foo", "Bar")
        .body(())
        .unwrap();

    // Simple assertion to verify response is created
    assert!(response.is_some());
}

