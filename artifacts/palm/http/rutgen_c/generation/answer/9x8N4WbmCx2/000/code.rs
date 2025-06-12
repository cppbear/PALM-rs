// Answer 0

#[test]
fn test_hdrname_custom_with_lower_true() {
    let buf: &[u8] = b"Test";
    let result = HdrName::custom(buf, true);
    assert_eq!(result.inner, Repr::Custom(MaybeLower { buf, lower: true }));
}

#[test]
fn test_hdrname_custom_with_lower_false() {
    let buf: &[u8] = b"Example";
    let result = HdrName::custom(buf, false);
    assert_eq!(result.inner, Repr::Custom(MaybeLower { buf, lower: false }));
}

#[test]
fn test_hdrname_custom_empty_buf() {
    let buf: &[u8] = b"";
    let result = HdrName::custom(buf, true);
    assert_eq!(result.inner, Repr::Custom(MaybeLower { buf, lower: true }));
}

#[test]
fn test_hdrname_custom_special_chars() {
    let buf: &[u8] = b"@#$%^&*";
    let result = HdrName::custom(buf, false);
    assert_eq!(result.inner, Repr::Custom(MaybeLower { buf, lower: false }));
}

