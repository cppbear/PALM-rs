// Answer 0

#[test]
fn test_headers_mut_success() {
    use crate::{Response, HeaderValue, Builder, StatusCode, Version};

    let mut builder = Builder::new()
        .status(StatusCode::OK)
        .version(Version::HTTP_11);
    
    {
        let headers = builder.headers_mut().unwrap();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("X-Example-Header", HeaderValue::from_static("value"));
    }
    
    let headers = builder.headers_ref().unwrap();
    assert_eq!(headers["Content-Type"], "application/json");
    assert_eq!(headers["X-Example-Header"], "value");
}

#[test]
fn test_headers_mut_failure() {
    use crate::{Builder, StatusCode, Version};

    let mut builder = Builder::new()
        .status(StatusCode::INTERNAL_SERVER_ERROR) // assuming this leads to an error in the builder
        .version(Version::HTTP_11);
    
    let headers = builder.headers_mut();
    assert!(headers.is_none());
}

