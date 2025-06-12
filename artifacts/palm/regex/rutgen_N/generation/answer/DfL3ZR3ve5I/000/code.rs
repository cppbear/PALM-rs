// Answer 0

#[test]
fn test_fmt_unprintable_start_and_end() {
    struct ClassUnicodeRange {
        start: char,
        end: char,
    }

    let range = ClassUnicodeRange {
        start: '\x00', // Control character
        end: '\x1F',   // Control character
    };

    let mut output = String::new();
    let result = std::fmt::write(&mut output, |f| range.fmt(f));
    
    assert!(result.is_ok());
    assert!(output.contains("start: 0x0"));
    assert!(output.contains("end: 0x1F"));
}

#[test]
fn test_fmt_printable_start_and_end() {
    struct ClassUnicodeRange {
        start: char,
        end: char,
    }

    let range = ClassUnicodeRange {
        start: 'A', // Printable character
        end: 'Z',   // Printable character
    };

    let mut output = String::new();
    let result = std::fmt::write(&mut output, |f| range.fmt(f));
    
    assert!(result.is_ok());
    assert!(output.contains("start: A"));
    assert!(output.contains("end: Z"));
}

#[test]
fn test_fmt_mixed_start_and_end() {
    struct ClassUnicodeRange {
        start: char,
        end: char,
    }

    let range = ClassUnicodeRange {
        start: '\x08', // Control character
        end: 'B',      // Printable character
    };

    let mut output = String::new();
    let result = std::fmt::write(&mut output, |f| range.fmt(f));
    
    assert!(result.is_ok());
    assert!(output.contains("start: 0x8"));
    assert!(output.contains("end: B"));
}

