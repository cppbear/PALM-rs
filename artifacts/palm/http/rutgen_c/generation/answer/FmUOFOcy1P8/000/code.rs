// Answer 0

#[test]
fn test_fmt_with_valid_status_code() {
    struct TestStatusCode(pub StatusCode);
    
    let status_code = StatusCode(unsafe { NonZeroU16::new_unchecked(200) }); // 200 OK
    let expected = "200 OK";
    
    let mut output = String::new();
    let result = write!(&mut output, "{}", TestStatusCode(status_code));
    
    assert!(result.is_ok());
    assert_eq!(output.trim(), expected);
}

#[test]
fn test_fmt_with_unknown_status_code() {
    struct TestStatusCode(pub StatusCode);
    
    let status_code = StatusCode(unsafe { NonZeroU16::new_unchecked(999) }); // Unknown status
    let expected = "999 <unknown status code>";
    
    let mut output = String::new();
    let result = write!(&mut output, "{}", TestStatusCode(status_code));
    
    assert!(result.is_ok());
    assert_eq!(output.trim(), expected);
}

#[test]
fn test_fmt_with_informational_status_code() {
    struct TestStatusCode(pub StatusCode);
    
    let status_code = StatusCode(unsafe { NonZeroU16::new_unchecked(100) }); // 100 Continue
    let expected = "100 Continue";
    
    let mut output = String::new();
    let result = write!(&mut output, "{}", TestStatusCode(status_code));
    
    assert!(result.is_ok());
    assert_eq!(output.trim(), expected);
}

#[test]
fn test_fmt_with_client_error_status_code() {
    struct TestStatusCode(pub StatusCode);
    
    let status_code = StatusCode(unsafe { NonZeroU16::new_unchecked(404) }); // 404 Not Found
    let expected = "404 Not Found";
    
    let mut output = String::new();
    let result = write!(&mut output, "{}", TestStatusCode(status_code));
    
    assert!(result.is_ok());
    assert_eq!(output.trim(), expected);
}

