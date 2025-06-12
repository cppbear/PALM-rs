// Answer 0

#[derive(Debug)]
struct ClassBytesRange {
    start: u32,
    end: u32,
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
fn test_class_bytes_range_fmt_both_greater_than_127() {
    let range = ClassBytesRange { start: 128, end: 255 };
    let mut output = String::new();
    let result = range.fmt(&mut std::fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert!(output.contains("start: 128"));
    assert!(output.contains("end: 255"));
}

#[test]
fn test_class_bytes_range_fmt_start_greater_than_127_end_equal_to_128() {
    let range = ClassBytesRange { start: 130, end: 128 };
    let mut output = String::new();
    let result = range.fmt(&mut std::fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert!(output.contains("start: 130"));
    assert!(output.contains("end: 128"));
}

#[test]
fn test_class_bytes_range_fmt_both_zero() {
    let range = ClassBytesRange { start: 128, end: 127 };
    let mut output = String::new();
    let result = range.fmt(&mut std::fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert!(output.contains("start: 128"));
    assert!(output.contains("end: 127"));
}

