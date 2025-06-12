// Answer 0

#[test]
fn test_body_with_unit_type() {
    struct ResponseBuilder {
        inner: Result<(), &'static str>,
    }

    impl ResponseBuilder {
        pub fn builder() -> Self {
            ResponseBuilder {
                inner: Ok(()),
            }
        }

        pub fn body<T>(self, body: T) -> Result<Response<T>, &'static str> {
            self.inner.map(move |_| Response { head: (), body })
        }
    }

    struct Response<T> {
        head: (),
        body: T,
    }

    let response = ResponseBuilder::builder().body(()).unwrap();
    assert_eq!(response.head, ());
}

#[test]
fn test_body_with_invalid_header() {
    struct ResponseBuilder {
        inner: Result<(), &'static str>,
    }

    impl ResponseBuilder {
        pub fn builder() -> Self {
            ResponseBuilder {
                inner: Err("Invalid header"),
            }
        }

        pub fn body<T>(self, body: T) -> Result<Response<T>, &'static str> {
            self.inner.map(move |_| Response { head: (), body })
        }
    }

    struct Response<T> {
        head: (),
        body: T,
    }

    let result = ResponseBuilder::builder().body(());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid header");
}

