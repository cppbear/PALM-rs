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
fn test_fmt_with_ascii_range() {
    let range = ClassBytesRange { start: 65, end: 90 }; // 'A' to 'Z'
    let mut output = String::new();
    {
        let formatter = &mut std::fmt::Formatter::new(&mut output);
        range.fmt(formatter).unwrap();
    }
    assert!(output.contains("start: 'A'"));
    assert!(output.contains("end: 'Z'"));
}

#[test]
fn test_fmt_with_non_ascii_range() {
    let range = ClassBytesRange { start: 200, end: 255 }; // Non-ASCII range
    let mut output = String::new();
    {
        let formatter = &mut std::fmt::Formatter::new(&mut output);
        range.fmt(formatter).unwrap();
    }
    assert!(output.contains("start: 200"));
    assert!(output.contains("end: 255"));
}

#[test]
fn test_fmt_with_mixed_range() {
    let range = ClassBytesRange { start: 100, end: 200 }; // Mixed range
    let mut output = String::new();
    {
        let formatter = &mut std::fmt::Formatter::new(&mut output);
        range.fmt(formatter).unwrap();
    }
    assert!(output.contains("start: 'd'")); // 100 is 'd'
    assert!(output.contains("end: 200"));
}

#[test]
fn test_fmt_with_exact_ascii_range() {
    let range = ClassBytesRange { start: 0, end: 127 }; // Lowest to highest ASCII
    let mut output = String::new();
    {
        let formatter = &mut std::fmt::Formatter::new(&mut output);
        range.fmt(formatter).unwrap();
    }
    assert!(output.contains("start: '\0'")); // 0 is NULL character
    assert!(output.contains("end: '\u{7f}'")); // 127 is DEL character
}

