// Answer 0

#[test]
fn test_fmt_with_start_and_end_at_7F() {
    let range = ClassBytesRange { start: 0x7F, end: 0x7F };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", range);
}

#[test]
fn test_fmt_with_start_at_7F_and_end_below_7F() {
    let range = ClassBytesRange { start: 0x7F, end: 0x7E };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", range);
}

#[test]
fn test_fmt_with_start_below_7F_and_end_at_7F() {
    let range = ClassBytesRange { start: 0x7E, end: 0x7F };
    let mut output = String::new();
    let result = write!(&mut output, "{:?}", range);
}

