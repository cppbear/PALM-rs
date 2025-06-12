// Answer 0

#[derive(Debug)]
struct ClassUnicodeRange {
    start: char,
    end: char,
}

impl ClassUnicodeRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let start = if !self.start.is_whitespace() && !self.start.is_control() {
            self.start.to_string()
        } else {
            format!("0x{:X}", self.start as u32)
        };
        let end = if !self.end.is_whitespace() && !self.end.is_control() {
            self.end.to_string()
        } else {
            format!("0x{:X}", self.end as u32)
        };
        f.debug_struct("ClassUnicodeRange")
         .field("start", &start)
         .field("end", &end)
         .finish()
    }
}

#[test]
fn test_fmt_with_control_characters() {
    let range = ClassUnicodeRange {
        start: '\u{0000}', // Start is a control character
        end: '\u{007F}',   // End is a control character
    };
    let mut output = std::fmt::Formatter::new();
    
    let result = range.fmt(&mut output);
    assert!(result.is_ok());
}

#[test]
fn test_fmt_with_non_whitespace_control_characters() {
    let range = ClassUnicodeRange {
        start: '\u{0001}', // Start is a non-whitespace control character
        end: '\u{001F}',   // End is a non-whitespace control character
    };
    let mut output = std::fmt::Formatter::new();
    
    let result = range.fmt(&mut output);
    assert!(result.is_ok());
}

#[test]
fn test_fmt_with_boundary_control_characters() {
    let range_start = ClassUnicodeRange {
        start: '\u{0000}', // Lower boundary control character
        end: '\u{007F}',   // Upper boundary control character
    };
    let mut output_start = std::fmt::Formatter::new();
    
    let result_start = range_start.fmt(&mut output_start);
    assert!(result_start.is_ok());

    let range_end = ClassUnicodeRange {
        start: '\u{007E}', // Just above control character
        end: '\u{007F}',   // Just at control character before whitespace
    };
    let mut output_end = std::fmt::Formatter::new();
    
    let result_end = range_end.fmt(&mut output_end);
    assert!(result_end.is_ok());
}

