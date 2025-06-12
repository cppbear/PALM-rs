// Answer 0

#[test]
fn body_mut_updates_body() {
    let mut request = Request::new(String::from("initial body"));
    request.body_mut().push_str(" updated");
    assert_eq!(request.body(), &"initial body updated");
}

#[test]
fn body_mut_empty_body() {
    let mut request: Request<String> = Request::new(String::new());
    request.body_mut().push_str("new body");
    assert!(!request.body().is_empty());
    assert_eq!(request.body(), &"new body");
}

#[test]
fn body_mut_multiple_updates() {
    let mut request: Request<String> = Request::new(String::from("start"));
    request.body_mut().push_str(" first update");
    request.body_mut().push_str(" second update");
    assert_eq!(request.body(), &"start first update second update");
}

