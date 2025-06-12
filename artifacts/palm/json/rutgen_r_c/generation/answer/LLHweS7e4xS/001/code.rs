// Answer 0

#[test]
fn test_with_indent_default() {
    struct TestFormatter<'a> {
        formatter: PrettyFormatter<'a>,
    }
    
    let formatter = TestFormatter {
        formatter: PrettyFormatter::with_indent(b"  "),
    };

    assert_eq!(formatter.formatter.current_indent, 0);
    assert_eq!(formatter.formatter.has_value, false);
    assert_eq!(formatter.formatter.indent, b"  ");
}

#[test]
fn test_with_indent_empty() {
    struct TestFormatter<'a> {
        formatter: PrettyFormatter<'a>,
    }
    
    let formatter = TestFormatter {
        formatter: PrettyFormatter::with_indent(b""),
    };

    assert_eq!(formatter.formatter.current_indent, 0);
    assert_eq!(formatter.formatter.has_value, false);
    assert_eq!(formatter.formatter.indent, b"");
}

#[test]
fn test_with_indent_one_byte() {
    struct TestFormatter<'a> {
        formatter: PrettyFormatter<'a>,
    }
    
    let formatter = TestFormatter {
        formatter: PrettyFormatter::with_indent(b"a"),
    };

    assert_eq!(formatter.formatter.current_indent, 0);
    assert_eq!(formatter.formatter.has_value, false);
    assert_eq!(formatter.formatter.indent, b"a");
}

#[test]
fn test_with_indent_max_bytes() {
    struct TestFormatter<'a> {
        formatter: PrettyFormatter<'a>,
    }

    const MAX_INDENT: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    
    let formatter = TestFormatter {
        formatter: PrettyFormatter::with_indent(MAX_INDENT),
    };

    assert_eq!(formatter.formatter.current_indent, 0);
    assert_eq!(formatter.formatter.has_value, false);
    assert_eq!(formatter.formatter.indent, MAX_INDENT);
}

