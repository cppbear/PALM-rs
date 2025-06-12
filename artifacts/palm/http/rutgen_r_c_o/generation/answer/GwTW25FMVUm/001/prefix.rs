// Answer 0

#[test]
fn test_builder_with_get_method() {
    let request = Request::builder()
        .method("GET")
        .uri("http://example.com")
        .header("Content-Type", "application/json")
        .body(())
        .unwrap();
}

#[test]
fn test_builder_with_post_method() {
    let request = Request::builder()
        .method("POST")
        .uri("https://example.com")
        .header("X-Custom-Foo", "Bar")
        .body("some body content")
        .unwrap();
}

#[test]
fn test_builder_with_put_method() {
    let request = Request::builder()
        .method("PUT")
        .uri("ftp://example.com")
        .header("Authorization", "Bearer token")
        .body(())
        .unwrap();
}

#[test]
fn test_builder_with_delete_method() {
    let request = Request::builder()
        .method("DELETE")
        .uri("http://localhost:8080")
        .header("Content-Type", "text/html")
        .body("some body content")
        .unwrap();
}

#[test]
fn test_builder_with_options_method() {
    let request = Request::builder()
        .method("OPTIONS")
        .uri("http://example.com")
        .header("X-Custom-Foo", "Bar")
        .body(())
        .unwrap();
}

#[test]
fn test_builder_with_head_method() {
    let request = Request::builder()
        .method("HEAD")
        .uri("https://example.com")
        .header("Authorization", "Bearer token")
        .body("some body content")
        .unwrap();
}

#[test]
fn test_builder_with_connect_method() {
    let request = Request::builder()
        .method("CONNECT")
        .uri("http://localhost:8080")
        .header("Content-Type", "application/json")
        .body(())
        .unwrap();
}

#[test]
fn test_builder_with_patch_method() {
    let request = Request::builder()
        .method("PATCH")
        .uri("ftp://example.com")
        .header("X-Custom-Foo", "Bar")
        .body("some body content")
        .unwrap();
}

#[test]
fn test_builder_with_trace_method() {
    let request = Request::builder()
        .method("TRACE")
        .uri("http://example.com")
        .header("Authorization", "Bearer token")
        .body(())
        .unwrap();
}

#[test]
#[should_panic]
fn test_builder_with_invalid_uri() {
    let request = Request::builder()
        .method("GET")
        .uri("invalid-uri")
        .header("Content-Type", "application/json")
        .body(())
        .unwrap();
}

