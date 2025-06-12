// Answer 0

#[test]
fn test_options_builder_with_valid_uri() {
    use http::*;
    
    let uri = "https://www.rust-lang.org/";
    let request = Request::options(uri)
        .body(())
        .unwrap();
    
    assert_eq!(*request.method(), Method::OPTIONS);
}

#[test]
#[should_panic]
fn test_options_builder_with_invalid_uri() {
    use http::*;
    
    let uri = "invalid_uri";
    let _request = Request::options(uri)
        .body(())
        .unwrap(); // This should panic due to invalid URI conversion
}

#[test]
fn test_options_builder_edge_case_empty_uri() {
    use http::*;

    let uri = "";
    let result = Request::options(uri)
        .body(());

    assert!(result.is_err()); // Expecting an error due to empty URI
}

