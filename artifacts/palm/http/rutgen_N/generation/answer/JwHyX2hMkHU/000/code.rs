// Answer 0

#[derive(Debug)]
struct Response<T> {
    body: T,
    head: String,
}

impl<T> Response<T> {
    fn builder() -> ResponseBuilder<T> {
        ResponseBuilder::default()
    }

    fn body(&self) -> &T {
        &self.body
    }
}

#[derive(Debug, Default)]
struct ResponseBuilder<T> {
    body: Option<T>,
}

impl<T> ResponseBuilder<T> {
    fn body(mut self, body: T) -> Result<Response<T>, String> {
        self.body = Some(body);
        Ok(Response {
            body: self.body.take().expect("Body should be set"),
            head: String::new(),
        })
    }
}

impl<T> Response<T> {
    pub fn map<F, U>(self, f: F) -> Response<U>
    where
        F: FnOnce(T) -> U,
    {
        Response {
            body: f(self.body),
            head: self.head,
        }
    }
}

#[test]
fn test_map_with_string_body() {
    let response = Response::builder().body("some string").unwrap();
    let mapped_response: Response<&[u8]> = response.map(|b| {
        assert_eq!(b, "some string");
        b.as_bytes()
    });
    assert_eq!(mapped_response.body(), &"some string".as_bytes());
}

#[test]
fn test_map_with_numeric_body() {
    let response = Response::builder().body(42).unwrap();
    let mapped_response: Response<i32> = response.map(|b| b * 2);
    assert_eq!(mapped_response.body(), &84);
}

#[test]
fn test_map_with_empty_body() {
    let response = Response::builder().body("").unwrap();
    let mapped_response: Response<&[u8]> = response.map(|b| {
        assert_eq!(b, "");
        b.as_bytes()
    });
    assert_eq!(mapped_response.body(), &"".as_bytes());
}

