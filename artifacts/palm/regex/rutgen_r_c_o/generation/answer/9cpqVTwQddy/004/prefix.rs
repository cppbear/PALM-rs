// Answer 0

#[test]
fn test_fmt_class_ascii_word_not_negated() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut output };
    
    let ast = ClassAscii {
        span: Span::default(),
        kind: ClassAsciiKind::Word,
        negated: false,
    };
    
    writer.fmt_class_ascii(&ast).unwrap();
}

#[test]
fn test_fmt_class_ascii_word_negated() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut output };
    
    let ast = ClassAscii {
        span: Span::default(),
        kind: ClassAsciiKind::Word,
        negated: true,
    };
    
    writer.fmt_class_ascii(&ast).unwrap();
}

#[test]
fn test_fmt_class_ascii_alpha_not_negated() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut output };
    
    let ast = ClassAscii {
        span: Span::default(),
        kind: ClassAsciiKind::Alpha,
        negated: false,
    };
    
    writer.fmt_class_ascii(&ast).unwrap();
}

#[test]
fn test_fmt_class_ascii_alpha_negated() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut output };
    
    let ast = ClassAscii {
        span: Span::default(),
        kind: ClassAsciiKind::Alpha,
        negated: true,
    };
    
    writer.fmt_class_ascii(&ast).unwrap();
}

#[test]
fn test_fmt_class_ascii_digit_not_negated() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut output };
    
    let ast = ClassAscii {
        span: Span::default(),
        kind: ClassAsciiKind::Digit,
        negated: false,
    };
    
    writer.fmt_class_ascii(&ast).unwrap();
}

#[test]
fn test_fmt_class_ascii_digit_negated() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut output };
    
    let ast = ClassAscii {
        span: Span::default(),
        kind: ClassAsciiKind::Digit,
        negated: true,
    };
    
    writer.fmt_class_ascii(&ast).unwrap();
}

