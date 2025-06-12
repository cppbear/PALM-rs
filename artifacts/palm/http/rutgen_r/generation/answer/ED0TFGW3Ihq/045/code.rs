// Answer 0

#[test]
fn test_parse_exact_http_false_https_true_other_invalid_scheme() {
    let s = b"invalid_scheme"; // does not match b"http"
    // Prepare SCHEME_CHARS array for the test
    const SCHEME_CHARS: [u8; 256] = {
        let mut arr = [0; 256];
        arr[b'a' as usize] = 1;
        arr[b'b' as usize] = 1;
        arr[b':' as usize] = b':'; // Invokes an error when this character is found
        // Initialize other ranges as necessary
        arr
    };

    let result = parse_exact(s);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.kind(), ErrorKind::InvalidScheme);
    }
}

#[test]
fn test_parse_exact_https() {
    let s = b"https"; // matches b"https"
    let result = parse_exact(s);
    assert!(result.is_ok());
}

#[test]
fn test_parse_exact_length_boundary_invalid_scheme() {
    let s = b"toolong"; // does not match b"http" or b"https", has valid length
    const MAX_SCHEME_LEN: usize = 8; // defining max length for the test
    // Using the same SCHEME_CHARS array setup
    const SCHEME_CHARS: [u8; 256] = {
        let mut arr = [0; 256];
        arr[b't' as usize] = 1;
        arr[b'o' as usize] = 1;
        arr[b'l' as usize] = 1;
        arr[b'1' as usize] = 1;
        arr[b'g' as usize] = 1;
        arr[b'2' as usize] = 1;
        arr[b'3' as usize] = 1;
        arr[b':' as usize] = b':'; // causes an invalid scheme error
        arr
    };

    let result = parse_exact(s);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.kind(), ErrorKind::InvalidScheme);
    }
}

#[test]
fn test_parse_exact_length_barrier() {
    let s = b"validscheme"; // length just over the maximum allowed
    const MAX_SCHEME_LEN: usize = 12; // defines just out of bounds
    // Using the same SCHEME_CHARS array setup
    const SCHEME_CHARS: [u8; 256] = {
        let mut arr = [0; 256];
        arr[b'v' as usize] = 1;
        arr[b'a' as usize] = 1;
        arr[b'l' as usize] = 1;
        arr[b'i' as usize] = 1;
        arr[b'd' as usize] = 1;
        arr[b's' as usize] = 1;
        arr[b'h' as usize] = 1;
        arr[b'm' as usize] = 1;
        arr[b'e' as usize] = 1;
        arr[b':' as usize] = b':'; // causes an invalid scheme error
        arr
    };

    let result = parse_exact(s);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.kind(), ErrorKind::SchemeTooLong);
    }
}

