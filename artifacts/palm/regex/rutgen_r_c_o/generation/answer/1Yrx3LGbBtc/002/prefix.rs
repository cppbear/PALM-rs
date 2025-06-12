// Answer 0

#[test]
fn test_fmt_class_perl_word_non_negated() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };
    let ast = ClassPerl {
        span: Span::default(),
        kind: ClassPerlKind::Word,
        negated: false,
    };
    writer.fmt_class_perl(&ast);
}

#[test]
fn test_fmt_class_perl_digit_non_negated() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };
    let ast = ClassPerl {
        span: Span::default(),
        kind: ClassPerlKind::Digit,
        negated: false,
    };
    writer.fmt_class_perl(&ast);
}

#[test]
fn test_fmt_class_perl_space_non_negated() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };
    let ast = ClassPerl {
        span: Span::default(),
        kind: ClassPerlKind::Space,
        negated: false,
    };
    writer.fmt_class_perl(&ast);
}

#[test]
fn test_fmt_class_perl_word_negated() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };
    let ast = ClassPerl {
        span: Span::default(),
        kind: ClassPerlKind::Word,
        negated: true,
    };
    writer.fmt_class_perl(&ast);
}

#[test]
fn test_fmt_class_perl_digit_negated() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };
    let ast = ClassPerl {
        span: Span::default(),
        kind: ClassPerlKind::Digit,
        negated: true,
    };
    writer.fmt_class_perl(&ast);
}

#[test]
fn test_fmt_class_perl_space_negated() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };
    let ast = ClassPerl {
        span: Span::default(),
        kind: ClassPerlKind::Space,
        negated: true,
    };
    writer.fmt_class_perl(&ast);
}

