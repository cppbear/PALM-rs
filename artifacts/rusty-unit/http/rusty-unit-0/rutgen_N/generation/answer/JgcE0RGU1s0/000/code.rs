// Answer 0

#[derive(Debug)]
struct Request<T> {
    head: String,
    body: T,
}

#[derive(Debug)]
struct RequestBuilder {
    head: String,
}

impl RequestBuilder {
    fn new() -> Self {
        Self {
            head: String::new(),
        }
    }

    fn header(mut self, key: &str, value: &str) -> Result<Self, String> {
        // Simulating header validation
        if value.contains('\r') || value.contains('\n') {
            return Err("Invalid header value".to_string());
        }
        self.head.push_str(&format!("{}: {}\n", key, value));
        Ok(self)
    }

    fn body<T>(self, body: T) -> Result<Request<T>, String> {
        if self.head.is_empty() {
            return Err("No headers set".to_string());
        }
        Ok(Request { head: self.head, body })
    }
}

#[test]
fn test_body_with_valid_header() {
    let builder = RequestBuilder::new();
    let request = builder.header("Content-Type", "application/json")
                         .unwrap()
                         .body("Hello, World!")
                         .unwrap();

    assert_eq!(request.head, "Content-Type: application/json\n");
    assert_eq!(request.body, "Hello, World!");
}

#[test]
#[should_panic(expected = "No headers set")]
fn test_body_without_headers() {
    let builder = RequestBuilder::new();
    let _request = builder.body("Hello, World!").unwrap();
}

#[test]
#[should_panic(expected = "Invalid header value")]
fn test_body_with_invalid_header() {
    let builder = RequestBuilder::new();
    let _request = builder.header("Foo", "Bar\r\n").unwrap()
                          .body("Hello, World!").unwrap();
}

