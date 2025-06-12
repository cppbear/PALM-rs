// Answer 0

#[test]
fn test_fmt_class_ascii_word_negated() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    
    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Word,
        negated: true,
    };
    
    writer.fmt_class_ascii(&ast);
}

#[test]
fn test_fmt_class_ascii_word_not_negated() {
    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    
    let ast = ast::ClassAscii {
        span: Span::default(),
        kind: ast::ClassAsciiKind::Word,
        negated: false,
    };
    
    writer.fmt_class_ascii(&ast);
}

