// Answer 0

#[derive(Debug, PartialEq)]
struct MaybeLower<'a> {
    buf: &'a [u8],
    lower: bool,
}

#[derive(Debug, PartialEq)]
enum Repr<'a> {
    Custom(MaybeLower<'a>),
}

#[derive(Debug, PartialEq)]
struct HdrName<'a> {
    inner: Repr<'a>,
}

fn custom<'a>(buf: &'a [u8], lower: bool) -> HdrName<'a> {
    HdrName {
        inner: Repr::Custom(MaybeLower { buf, lower }),
    }
}

#[test]
fn test_custom_with_non_empty_buffer_and_lower_true() {
    let buffer: &[u8] = b"Example-Header";
    let result = custom(buffer, true);
    assert_eq!(
        result,
        HdrName {
            inner: Repr::Custom(MaybeLower {
                buf: buffer,
                lower: true,
            })
        }
    );
}

#[test]
fn test_custom_with_non_empty_buffer_and_lower_false() {
    let buffer: &[u8] = b"Example-Header";
    let result = custom(buffer, false);
    assert_eq!(
        result,
        HdrName {
            inner: Repr::Custom(MaybeLower {
                buf: buffer,
                lower: false,
            })
        }
    );
}

#[test]
fn test_custom_with_empty_buffer_and_lower_true() {
    let buffer: &[u8] = b"";
    let result = custom(buffer, true);
    assert_eq!(
        result,
        HdrName {
            inner: Repr::Custom(MaybeLower {
                buf: buffer,
                lower: true,
            })
        }
    );
}

#[test]
fn test_custom_with_empty_buffer_and_lower_false() {
    let buffer: &[u8] = b"";
    let result = custom(buffer, false);
    assert_eq!(
        result,
        HdrName {
            inner: Repr::Custom(MaybeLower {
                buf: buffer,
                lower: false,
            })
        }
    );
}

