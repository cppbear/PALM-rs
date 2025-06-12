// Answer 0

#[test]
fn test_fmt_with_newline() {
    struct TestBytesRef<'a> {
        bytes: BytesRef<'a>,
    }

    let input = b"Hello\nWorld";
    let test_bytes_ref = TestBytesRef {
        bytes: BytesRef(input),
    };

    let mut output = vec![];
    let result = test_bytes_ref.bytes.fmt(&mut Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8_lossy(&output), "b\"Hello\\nWorld\"");
}

#[test]
fn test_fmt_with_carriage_return() {
    struct TestBytesRef<'a> {
        bytes: BytesRef<'a>,
    }

    let input = b"Hello\rWorld";
    let test_bytes_ref = TestBytesRef {
        bytes: BytesRef(input),
    };

    let mut output = vec![];
    let result = test_bytes_ref.bytes.fmt(&mut Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8_lossy(&output), "b\"Hello\\rWorld\"");
}

#[test]
fn test_fmt_with_tab() {
    struct TestBytesRef<'a> {
        bytes: BytesRef<'a>,
    }

    let input = b"Hello\tWorld";
    let test_bytes_ref = TestBytesRef {
        bytes: BytesRef(input),
    };

    let mut output = vec![];
    let result = test_bytes_ref.bytes.fmt(&mut Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8_lossy(&output), "b\"Hello\\tWorld\"");
}

#[test]
fn test_fmt_with_backslash() {
    struct TestBytesRef<'a> {
        bytes: BytesRef<'a>,
    }

    let input = b"Hello\\World";
    let test_bytes_ref = TestBytesRef {
        bytes: BytesRef(input),
    };

    let mut output = vec![];
    let result = test_bytes_ref.bytes.fmt(&mut Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8_lossy(&output), "b\"Hello\\\\World\"");
}

#[test]
fn test_fmt_with_quote() {
    struct TestBytesRef<'a> {
        bytes: BytesRef<'a>,
    }

    let input = b"Hello\"World";
    let test_bytes_ref = TestBytesRef {
        bytes: BytesRef(input),
    };

    let mut output = vec![];
    let result = test_bytes_ref.bytes.fmt(&mut Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8_lossy(&output), "b\"Hello\\\"World\"");
}

#[test]
fn test_fmt_with_null() {
    struct TestBytesRef<'a> {
        bytes: BytesRef<'a>,
    }

    let input = b"Hello\0World";
    let test_bytes_ref = TestBytesRef {
        bytes: BytesRef(input),
    };

    let mut output = vec![];
    let result = test_bytes_ref.bytes.fmt(&mut Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8_lossy(&output), "b\"Hello\\0World\"");
}

#[test]
fn test_fmt_with_printable_ascii() {
    struct TestBytesRef<'a> {
        bytes: BytesRef<'a>,
    }

    let input = b"Hello World!";
    let test_bytes_ref = TestBytesRef {
        bytes: BytesRef(input),
    };

    let mut output = vec![];
    let result = test_bytes_ref.bytes.fmt(&mut Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8_lossy(&output), "b\"Hello World!\"");
}

#[test]
fn test_fmt_with_non_printable_ascii() {
    struct TestBytesRef<'a> {
        bytes: BytesRef<'a>,
    }

    let input = b"Hello\x01World";
    let test_bytes_ref = TestBytesRef {
        bytes: BytesRef(input),
    };

    let mut output = vec![];
    let result = test_bytes_ref.bytes.fmt(&mut Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8_lossy(&output), "b\"Hello\\x01World\"");
}

