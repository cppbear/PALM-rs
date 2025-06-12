// Answer 0

#[test]
fn test_uri_display() {
    use std::fmt::Write; // Import for easy string handling in tests
    
    // Initialize necessary structures
    let scheme = Scheme { inner: Scheme2::from_str("http").unwrap() }; // Assuming 'http' is valid
    let authority = Authority { data: ByteStr::from_static(b"www.example.com") }; // Valid authority
    let path_and_query = PathAndQuery {
        data: ByteStr::from_static(b"/path/to/resource"), // Valid path
        query: 0, // Boundary condition, no query
    };
    
    let uri = Uri { scheme, authority, path_and_query };

    // Use a string to hold the output
    let mut output = String::new();
    
    // Calling `fmt` logic (through Debug trait)
    let result = uri.fmt(&mut output);
    
    // Assert that the result is Ok
    assert!(result.is_ok());
    
    // Check that the output matches expected format
    let expected_output = "Uri { scheme: \"http\", authority: \"www.example.com\", path_and_query: \"/path/to/resource\" }"; // Example expected output
    assert_eq!(output, expected_output);
}

#[test]
#[should_panic]
fn test_uri_display_invalid_scheme() {
    // Attempting to create a Uri with an invalid scheme that would panic if validated
    let scheme = Scheme { inner: Scheme2::from_str("invalid_scheme").unwrap() }; // Invalid scheme to trigger panic
    let authority = Authority { data: ByteStr::from_static(b"www.example.com") };
    let path_and_query = PathAndQuery {
        data: ByteStr::from_static(b"/path/to/resource"),
        query: 0,
    };

    let uri = Uri { scheme, authority, path_and_query };

    // This call is expected to panic due to invalid scheme
    let _ = uri.fmt(&mut String::new());
}

