// Answer 0

#[test]
fn test_options_valid_uri() {
    use crate::Uri;
    
    let valid_uri = "https://www.rust-lang.org/";
    let builder = Request::options(valid_uri);
    
    // Check that the method is OPTIONS
    assert_eq!(builder.method_ref().unwrap(), &Method::OPTIONS);
    
    // Check that the URI is correctly set
    assert_eq!(builder.uri_ref().unwrap().to_string(), valid_uri);
}

#[test]
#[should_panic]
fn test_options_empty_uri() {
    use crate::Uri;

    let empty_uri = "";
    let builder = Request::options(empty_uri);
    
    // This will panic because an empty string shouldn't be converted into a valid Uri
    let _ = builder.uri_ref().unwrap();
}

#[test]
#[should_panic]
fn test_options_invalid_uri() {
    use crate::Uri;

    let invalid_uri = "invalid_uri";
    let builder = Request::options(invalid_uri);
    
    // This will panic because an invalid string shouldn't be convertible into a valid Uri
    let _ = builder.uri_ref().unwrap();
}

#[test]
fn test_options_non_ascii_uri() {
    use crate::Uri;

    let non_ascii_uri = "https://example.com/路径"; // Chinese characters as an example
    let builder = Request::options(non_ascii_uri);
    
    // Check that the method is OPTIONS
    assert_eq!(builder.method_ref().unwrap(), &Method::OPTIONS);
    
    // Check that the URI is correctly set
    assert_eq!(builder.uri_ref().unwrap().to_string(), non_ascii_uri);
}

