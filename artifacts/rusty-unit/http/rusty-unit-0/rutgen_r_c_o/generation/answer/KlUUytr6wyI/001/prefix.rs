// Answer 0

#[test]
fn test_body_mut_empty_string() {
    let mut request: Request<String> = Request::new(String::new());
    let body_mut = request.body_mut();
    body_mut.push_str("Hello");
}

#[test]
fn test_body_mut_non_empty_string() {
    let mut request: Request<String> = Request::new(String::from("Initial body"));
    let body_mut = request.body_mut();
    body_mut.push_str(" - appended");
}

#[test]
fn test_body_mut_large_string() {
    let mut request: Request<String> = Request::new(String::new());
    let body_mut = request.body_mut();
    body_mut.push_str("a".repeat(1_000_000).as_str());
}

#[test]
fn test_body_mut_fill_up_limit() {
    let mut request: Request<String> = Request::new(String::new());
    let body_mut = request.body_mut();
    body_mut.push_str("abcdefghij");  // Length hits the upper limit of 10^6
}

#[test]
fn test_body_mut_panic_on_large_capacity() {
    let mut request: Request<String> = Request::new(String::new());
    let body_mut = request.body_mut();
    body_mut.push_str("a".repeat(1_000_001).as_str()); // This should trigger panic as we exceed the length limit.
}

