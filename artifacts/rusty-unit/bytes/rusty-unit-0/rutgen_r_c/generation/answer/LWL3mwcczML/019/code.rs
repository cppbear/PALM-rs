// Answer 0

#[test]
fn test_bytes_ref_debug_output_with_newline() {
    struct BytesRefTest<'a>(&'a [u8]);
    
    let data = b"Hello\nWorld";
    let bytes_ref = BytesRefTest(data);
    
    let mut output = Vec::new();
    let result = bytes_ref.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output.as_slice(), b"b\"Hello\\nWorld\"");
}

#[test]
fn test_bytes_ref_debug_output_with_carriage_return() {
    struct BytesRefTest<'a>(&'a [u8]);
    
    let data = b"Hello\rWorld";
    let bytes_ref = BytesRefTest(data);
    
    let mut output = Vec::new();
    let result = bytes_ref.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output.as_slice(), b"b\"Hello\\rWorld\"");
}

#[test]
fn test_bytes_ref_debug_output_with_tab() {
    struct BytesRefTest<'a>(&'a [u8]);
    
    let data = b"Hello\tWorld";
    let bytes_ref = BytesRefTest(data);
    
    let mut output = Vec::new();
    let result = bytes_ref.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output.as_slice(), b"b\"Hello\\tWorld\"");
}

#[test]
fn test_bytes_ref_debug_output_with_escape() {
    struct BytesRefTest<'a>(&'a [u8]);
    
    let data = b"Hello\\World";
    let bytes_ref = BytesRefTest(data);
    
    let mut output = Vec::new();
    let result = bytes_ref.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output.as_slice(), b"b\"Hello\\\\World\"");
}

#[test]
fn test_bytes_ref_debug_output_with_quote() {
    struct BytesRefTest<'a>(&'a [u8]);
    
    let data = b"Hello\"World";
    let bytes_ref = BytesRefTest(data);
    
    let mut output = Vec::new();
    let result = bytes_ref.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output.as_slice(), b"b\"Hello\\\"World\"");
}

#[test]
fn test_bytes_ref_debug_output_with_non_printable() {
    struct BytesRefTest<'a>(&'a [u8]);
    
    let data = b"Hello\x01World";
    let bytes_ref = BytesRefTest(data);
    
    let mut output = Vec::new();
    let result = bytes_ref.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output.as_slice(), b"b\"Hello\\x01World\"");
}

#[test]
fn test_bytes_ref_debug_output_with_null_byte() {
    struct BytesRefTest<'a>(&'a [u8]);
    
    let data = b"Hello\0World";
    let bytes_ref = BytesRefTest(data);
    
    let mut output = Vec::new();
    let result = bytes_ref.fmt(&mut output);
    
    assert!(result.is_ok());
    assert_eq!(output.as_slice(), b"b\"Hello\\0World\"");
}

