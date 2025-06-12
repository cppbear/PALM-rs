// Answer 0

#[test]
fn test_custom_with_empty_buf() {
    let hdr_name = HdrName::custom(&[], false);
    match hdr_name.inner {
        Repr::Custom(MaybeLower { buf, lower }) => {
            assert_eq!(buf, &[]);
            assert_eq!(lower, false);
        },
        _ => panic!("Expected Repr::Custom variant"),
    }
}

#[test]
fn test_custom_with_non_empty_buf_lower_true() {
    let buf = b"example-header";
    let hdr_name = HdrName::custom(buf, true);
    match hdr_name.inner {
        Repr::Custom(MaybeLower { buf: returned_buf, lower }) => {
            assert_eq!(returned_buf, buf);
            assert_eq!(lower, true);
        },
        _ => panic!("Expected Repr::Custom variant"),
    }
}

#[test]
fn test_custom_with_non_empty_buf_lower_false() {
    let buf = b"another-header";
    let hdr_name = HdrName::custom(buf, false);
    match hdr_name.inner {
        Repr::Custom(MaybeLower { buf: returned_buf, lower }) => {
            assert_eq!(returned_buf, buf);
            assert_eq!(lower, false);
        },
        _ => panic!("Expected Repr::Custom variant"),
    }
}

