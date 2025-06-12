// Answer 0

#[test]
fn test_fmt_with_newline() {
    struct TestBytesRef<'a>(&'a [u8]);
    
    let data = b"Hello\nWorld";
    let bytes_ref = TestBytesRef(data);
    let mut output = String::new();
    
    let result = std::fmt::write(&mut output, |f| bytes_ref.fmt(f));
    assert!(result.is_ok());
    assert_eq!(output, "b\"Hello\\nWorld\"");
}

#[test]
fn test_fmt_with_control_chars() {
    struct TestBytesRef<'a>(&'a [u8]);
    
    let data = b"Hello\x00World";
    let bytes_ref = TestBytesRef(data);
    let mut output = String::new();
    
    let result = std::fmt::write(&mut output, |f| bytes_ref.fmt(f));
    assert!(result.is_ok());
    assert_eq!(output, "b\"Hello\\0World\"");
}

#[test]
fn test_fmt_with_special_chars() {
    struct TestBytesRef<'a>(&'a [u8]);
    
    let data = b"Hello\\World\"Test";
    let bytes_ref = TestBytesRef(data);
    let mut output = String::new();
    
    let result = std::fmt::write(&mut output, |f| bytes_ref.fmt(f));
    assert!(result.is_ok());
    assert_eq!(output, "b\"Hello\\\\World\\\"Test\"");
}

#[should_panic]
fn test_fmt_with_cr() {
    struct TestBytesRef<'a>(&'a [u8]);
    
    let data = b"Hello\rWorld";
    let bytes_ref = TestBytesRef(data);
    let mut output = String::new();
    
    let _ = std::fmt::write(&mut output, |f| bytes_ref.fmt(f));
}

