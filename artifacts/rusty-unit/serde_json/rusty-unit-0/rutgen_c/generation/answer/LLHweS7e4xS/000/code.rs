// Answer 0

#[test]
fn test_pretty_formatter_with_indent() {
    struct TestFormatter<'a> {
        formatter: PrettyFormatter<'a>
    }

    let indent = b"   ";
    let formatter = TestFormatter {
        formatter: PrettyFormatter::with_indent(indent),
    };

    assert_eq!(formatter.formatter.current_indent, 0);
    assert_eq!(formatter.formatter.has_value, false);
    assert_eq!(formatter.formatter.indent, indent);
}

#[test]
fn test_pretty_formatter_new() {
    struct TestFormatter<'a> {
        formatter: PrettyFormatter<'a>
    }

    let formatter = TestFormatter {
        formatter: PrettyFormatter::new(),
    };

    assert_eq!(formatter.formatter.current_indent, 0);
    assert_eq!(formatter.formatter.has_value, false);
    assert_eq!(formatter.formatter.indent, b"  ");
}

