// Answer 0

#[derive(Debug)]
struct ClassBytesRange {
    start: u8,
    end: u8,
}

impl ClassBytesRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut debug = f.debug_struct("ClassBytesRange");
        if self.start <= 0x7F {
            debug.field("start", &(self.start as char));
        } else {
            debug.field("start", &self.start);
        }
        if self.end <= 0x7F {
            debug.field("end", &(self.end as char));
        } else {
            debug.field("end", &self.end);
        }
        debug.finish()
    }
}

#[test]
fn test_class_bytes_range_fmt_with_bound_start() {
    let mut output = String::new();
    let range = ClassBytesRange { start: 0x7F, end: 0x7F };
    range.fmt(&mut std::fmt::Formatter::new(&mut output)).unwrap();
    assert!(output.contains("start: '\u{7f}'"));
    assert!(output.contains("end: '\u{7f}'"));
}

#[test]
fn test_class_bytes_range_fmt_with_lower_bound_start_end() {
    let mut output = String::new();
    let range = ClassBytesRange { start: 0x00, end: 0x00 };
    range.fmt(&mut std::fmt::Formatter::new(&mut output)).unwrap();
    assert!(output.contains("start: '\u{0}'"));
    assert!(output.contains("end: '\u{0}'"));
}

#[test]
fn test_class_bytes_range_fmt_with_mixed_bounds() {
    let mut output = String::new();
    let range = ClassBytesRange { start: 0x00, end: 0x7F };
    range.fmt(&mut std::fmt::Formatter::new(&mut output)).unwrap();
    assert!(output.contains("start: '\u{0}'"));
    assert!(output.contains("end: '\u{7f}'"));
}

