// Answer 0

#[test]
fn test_fmt_class_ascii_xdigit_negated() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    
    let ast = ast::ClassAscii {
        span: Span::default(), // Assuming a default span or appropriate value
        kind: ast::ClassAsciiKind::Xdigit,
        negated: true,
    };
    
    writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_xdigit_non_negated() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    
    let ast = ast::ClassAscii {
        span: Span::default(), // Assuming a default span or appropriate value
        kind: ast::ClassAsciiKind::Xdigit,
        negated: false,
    };
    
    writer.fmt_class_ascii(&ast);
}

