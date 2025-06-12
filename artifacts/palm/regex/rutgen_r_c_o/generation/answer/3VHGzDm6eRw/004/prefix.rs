// Answer 0

#[test]
fn test_fmt_start_above_127_end_above_127() {
    let range = ClassBytesRange { start: 128, end: 130 };
    let mut output = vec![];
    let mut formatter = fmt::Formatter::new(&mut output);
    range.fmt(&mut formatter).unwrap();
}

#[test]
fn test_fmt_start_at_255_end_at_255() {
    let range = ClassBytesRange { start: 255, end: 255 };
    let mut output = vec![];
    let mut formatter = fmt::Formatter::new(&mut output);
    range.fmt(&mut formatter).unwrap();
}

#[test]
fn test_fmt_start_above_127_end_at_255() {
    let range = ClassBytesRange { start: 200, end: 255 };
    let mut output = vec![];
    let mut formatter = fmt::Formatter::new(&mut output);
    range.fmt(&mut formatter).unwrap();
}

#[test]
fn test_fmt_start_at_128_end_above_127() {
    let range = ClassBytesRange { start: 128, end: 250 };
    let mut output = vec![];
    let mut formatter = fmt::Formatter::new(&mut output);
    range.fmt(&mut formatter).unwrap();
}

#[test]
fn test_fmt_start_above_127_end_at_128() {
    let range = ClassBytesRange { start: 200, end: 128 };
    let mut output = vec![];
    let mut formatter = fmt::Formatter::new(&mut output);
    range.fmt(&mut formatter).unwrap();
}

