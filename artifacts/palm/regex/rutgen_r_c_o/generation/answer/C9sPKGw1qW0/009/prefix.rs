// Answer 0

#[test]
fn test_visit_class_set_item_post_range_valid() {
    use ast::{ClassSetItem, ClassSetRange, Literal, Span, LiteralKind};

    let start_literal = Literal {
        span: Span::default(),
        kind: LiteralKind::Verbatim,
        c: 'a',
    };
    
    let end_literal = Literal {
        span: Span::default(),
        kind: LiteralKind::Verbatim,
        c: 'z',
    };
    
    let range = ClassSetRange {
        span: Span::default(),
        start: start_literal,
        end: end_literal,
    };

    let class_set_item = ClassSetItem::Range(range);

    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };

    writer.visit_class_set_item_post(&class_set_item);
}

#[test]
fn test_visit_class_set_item_post_range_start_end_equal() {
    use ast::{ClassSetItem, ClassSetRange, Literal, Span, LiteralKind};

    let same_literal = Literal {
        span: Span::default(),
        kind: LiteralKind::Verbatim,
        c: 'm',
    };
    
    let range = ClassSetRange {
        span: Span::default(),
        start: same_literal.clone(),
        end: same_literal,
    };

    let class_set_item = ClassSetItem::Range(range);

    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };

    writer.visit_class_set_item_post(&class_set_item);
}

#[test]
fn test_visit_class_set_item_post_range_with_unicode() {
    use ast::{ClassSetItem, ClassSetRange, Literal, Span, LiteralKind};

    let start_literal = Literal {
        span: Span::default(),
        kind: LiteralKind::Verbatim,
        c: '\u{0030}', // '0'
    };
    
    let end_literal = Literal {
        span: Span::default(),
        kind: LiteralKind::Verbatim,
        c: '\u{0039}', // '9'
    };
    
    let range = ClassSetRange {
        span: Span::default(),
        start: start_literal,
        end: end_literal,
    };

    let class_set_item = ClassSetItem::Range(range);

    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };

    writer.visit_class_set_item_post(&class_set_item);
}

#[test]
fn test_visit_class_set_item_post_empty() {
    use ast::{ClassSetItem, Span};

    let class_set_item = ClassSetItem::Empty(Span::default());

    let mut output = String::new();
    let mut printer = Printer { _priv: () };
    let writer = Writer {
        printer: &mut printer,
        wtr: &mut output,
    };

    writer.visit_class_set_item_post(&class_set_item);
}

