// Answer 0

#[test]
fn test_fmt_class_perl_space_negated() {
    let mut buffer = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer {
        printer,
        wtr: &mut buffer,
    };
    
    let ast = ast::ClassPerl {
        span: Span::dummy(), // Assuming a dummy span function exists
        kind: ast::ClassPerlKind::Space,
        negated: true,
    };
    
    writer.fmt_class_perl(&ast).unwrap();
}

#[test]
fn test_fmt_class_perl_space_non_negated() {
    let mut buffer = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer {
        printer,
        wtr: &mut buffer,
    };
    
    let ast = ast::ClassPerl {
        span: Span::dummy(), // Assuming a dummy span function exists
        kind: ast::ClassPerlKind::Space,
        negated: false,
    };
    
    writer.fmt_class_perl(&ast).unwrap();
}

