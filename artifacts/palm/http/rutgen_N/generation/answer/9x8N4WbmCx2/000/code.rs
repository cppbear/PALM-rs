// Answer 0

#[derive(Debug)]
struct MaybeLower<'a> {
    buf: &'a [u8],
    lower: bool,
}

#[derive(Debug)]
enum Repr<'a> {
    Custom(MaybeLower<'a>),
}

struct HdrName<'a> {
    inner: Repr<'a>,
}

#[test]
fn test_custom_with_lower_true() {
    let buffer: &[u8] = b"Test-Header";
    let hdr = custom(buffer, true);
    if let Repr::Custom(maybe_lower) = hdr.inner {
        assert_eq!(maybe_lower.buf, buffer);
        assert!(maybe_lower.lower);
    } else {
        panic!("Expected Repr::Custom");
    }
}

#[test]
fn test_custom_with_lower_false() {
    let buffer: &[u8] = b"Another-Header";
    let hdr = custom(buffer, false);
    if let Repr::Custom(maybe_lower) = hdr.inner {
        assert_eq!(maybe_lower.buf, buffer);
        assert!(!maybe_lower.lower);
    } else {
        panic!("Expected Repr::Custom");
    }
}

#[test]
fn test_custom_with_empty_buffer() {
    let buffer: &[u8] = b"";
    let hdr = custom(buffer, true);
    if let Repr::Custom(maybe_lower) = hdr.inner {
        assert_eq!(maybe_lower.buf, buffer);
        assert!(maybe_lower.lower);
    } else {
        panic!("Expected Repr::Custom");
    }
}

