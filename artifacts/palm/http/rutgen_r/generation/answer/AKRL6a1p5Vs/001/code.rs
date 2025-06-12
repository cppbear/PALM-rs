// Answer 0

#[test]
fn test_header_valid_key_value() {
    use http::*;
    use http::header::HeaderValue;

    let req = Request::builder()
        .header("Accept", "text/html")
        .header("X-Custom-Foo", "bar")
        .body(())
        .unwrap();

    assert_eq!(req.headers.get("Accept").unwrap(), "text/html");
    assert_eq!(req.headers.get("X-Custom-Foo").unwrap(), "bar");
}

#[test]
#[should_panic]
fn test_header_invalid_key() {
    use http::*;

    let req = Request::builder()
        .header("", "value") // Empty key
        .body(())
        .unwrap();
}

#[test]
#[should_panic]
fn test_header_invalid_value() {
    use http::*;
    use http::header::HeaderValue;

    let req = Request::builder()
        .header("X-Custom-Foo", "INVALID_VALUE") // Simulated invalid HeaderValue generation
        .body(())
        .unwrap();
}

#[test]
fn test_header_multiple_appends() {
    use http::*;
    use http::header::HeaderValue;

    let req = Request::builder()
        .header("Accept", "application/json")
        .header("Accept", "text/html") // Multiple values for same key
        .body(())
        .unwrap();

    assert_eq!(req.headers.get("Accept").unwrap().to_str().unwrap(), "application/json, text/html");
}

#[test]
fn test_header_special_characters_in_key_and_value() {
    use http::*;
    
    let req = Request::builder()
        .header("X-Complex$Header", "Value_with_special_characters!@#") 
        .body(())
        .unwrap();

    assert_eq!(req.headers.get("X-Complex$Header").unwrap(), "Value_with_special_characters!@#");
}

