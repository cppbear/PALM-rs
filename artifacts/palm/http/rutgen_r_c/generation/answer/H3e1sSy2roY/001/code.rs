// Answer 0

#[test]
fn test_body_mut_initialization() {
    let mut response: Response<String> = Response::new(String::from("initial body"));
    let body_ref = response.body_mut();
    body_ref.push_str(" modified");
    assert_eq!(response.body(), "initial body modified");
}

#[test]
fn test_body_mut_empty_string() {
    let mut response: Response<String> = Response::new(String::new());
    let body_ref = response.body_mut();
    body_ref.push_str("hello world");
    assert!(!response.body().is_empty());
    assert_eq!(response.body(), "hello world");
}

#[test]
fn test_body_mut_panic_on_drop() {
    struct PanicOnDrop;
    impl Drop for PanicOnDrop {
        fn drop(&mut self) {
            panic!("Dropped!");
        }
    }

    let mut response: Response<PanicOnDrop> = Response::new(PanicOnDrop);
    let body_ref = response.body_mut(); // Body reference is created
    std::mem::drop(body_ref); // Dropping the reference should not panic
}

