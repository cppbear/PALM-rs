// Answer 0

#[test]
fn test_fmt_class_perl_word_negated() {
    let mut buffer = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut buffer };
    let ast = ast::ClassPerl {
        span: Span { start: 0, end: 1 },
        kind: ClassPerlKind::Word,
        negated: true,
    };
    writer.fmt_class_perl(&ast);
}

#[test]
fn test_fmt_class_perl_word_not_negated() {
    let mut buffer = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut buffer };
    let ast = ast::ClassPerl {
        span: Span { start: 0, end: 1 },
        kind: ClassPerlKind::Word,
        negated: false,
    };
    writer.fmt_class_perl(&ast);
}

#[test]
fn test_fmt_class_perl_space_negated() {
    let mut buffer = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut buffer };
    let ast = ast::ClassPerl {
        span: Span { start: 0, end: 1 },
        kind: ClassPerlKind::Space,
        negated: true,
    };
    writer.fmt_class_perl(&ast);
}

#[test]
fn test_fmt_class_perl_space_not_negated() {
    let mut buffer = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut buffer };
    let ast = ast::ClassPerl {
        span: Span { start: 0, end: 1 },
        kind: ClassPerlKind::Space,
        negated: false,
    };
    writer.fmt_class_perl(&ast);
}

#[test]
fn test_fmt_class_perl_digit_negated() {
    let mut buffer = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut buffer };
    let ast = ast::ClassPerl {
        span: Span { start: 0, end: 1 },
        kind: ClassPerlKind::Digit,
        negated: true,
    };
    writer.fmt_class_perl(&ast);
}

#[test]
fn test_fmt_class_perl_digit_not_negated() {
    let mut buffer = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut buffer };
    let ast = ast::ClassPerl {
        span: Span { start: 0, end: 1 },
        kind: ClassPerlKind::Digit,
        negated: false,
    };
    writer.fmt_class_perl(&ast);
}

