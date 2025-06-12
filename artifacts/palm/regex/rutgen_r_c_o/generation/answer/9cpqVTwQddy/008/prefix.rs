// Answer 0

#[test]
fn test_fmt_class_ascii_space_not_negated() {
    use std::fmt::Write;
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer {
        printer,
        wtr: &mut output,
    };
    let ast = ClassAscii {
        span: Span::default(), 
        kind: ClassAsciiKind::Space,
        negated: false,
    };
    writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_space_negated() {
    use std::fmt::Write;
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer {
        printer,
        wtr: &mut output,
    };
    let ast = ClassAscii {
        span: Span::default(), 
        kind: ClassAsciiKind::Space,
        negated: true,
    };
    writer.fmt_class_ascii(&ast);
}

