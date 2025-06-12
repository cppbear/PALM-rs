// Answer 0

#[test]
fn test_visit_class_set_item_post_ascii_negated() {
    let span = Span {}; // Initialize the span as required
    let class_ascii = ast::ClassAscii {
        span,
        kind: ast::ClassAsciiKind::Alnum,
        negated: true,
    };
    let ast_item = ast::ClassSetItem::Ascii(class_ascii);
    let mut writer = Printer { _priv: () };
    let mut visitor = Writer { printer: &mut writer, wtr: String::new() };
    let _ = visitor.visit_class_set_item_post(&ast_item);
}

#[test]
fn test_visit_class_set_item_post_ascii_not_negated() {
    let span = Span {}; // Initialize the span as required
    let class_ascii = ast::ClassAscii {
        span,
        kind: ast::ClassAsciiKind::Alpha,
        negated: false,
    };
    let ast_item = ast::ClassSetItem::Ascii(class_ascii);
    let mut writer = Printer { _priv: () };
    let mut visitor = Writer { printer: &mut writer, wtr: String::new() };
    let _ = visitor.visit_class_set_item_post(&ast_item);
}

#[test]
fn test_visit_class_set_item_post_ascii_special_characters_negated() {
    let span = Span {}; // Initialize the span as required
    let class_ascii = ast::ClassAscii {
        span,
        kind: ast::ClassAsciiKind::Punct,
        negated: true,
    };
    let ast_item = ast::ClassSetItem::Ascii(class_ascii);
    let mut writer = Printer { _priv: () };
    let mut visitor = Writer { printer: &mut writer, wtr: String::new() };
    let _ = visitor.visit_class_set_item_post(&ast_item);
}

#[test]
fn test_visit_class_set_item_post_ascii_special_characters_not_negated() {
    let span = Span {}; // Initialize the span as required
    let class_ascii = ast::ClassAscii {
        span,
        kind: ast::ClassAsciiKind::Space,
        negated: false,
    };
    let ast_item = ast::ClassSetItem::Ascii(class_ascii);
    let mut writer = Printer { _priv: () };
    let mut visitor = Writer { printer: &mut writer, wtr: String::new() };
    let _ = visitor.visit_class_set_item_post(&ast_item);
}

#[test]
fn test_visit_class_set_item_post_ascii_digit_negated() {
    let span = Span {}; // Initialize the span as required
    let class_ascii = ast::ClassAscii {
        span,
        kind: ast::ClassAsciiKind::Digit,
        negated: true,
    };
    let ast_item = ast::ClassSetItem::Ascii(class_ascii);
    let mut writer = Printer { _priv: () };
    let mut visitor = Writer { printer: &mut writer, wtr: String::new() };
    let _ = visitor.visit_class_set_item_post(&ast_item);
}

#[test]
fn test_visit_class_set_item_post_ascii_digit_not_negated() {
    let span = Span {}; // Initialize the span as required
    let class_ascii = ast::ClassAscii {
        span,
        kind: ast::ClassAsciiKind::Digit,
        negated: false,
    };
    let ast_item = ast::ClassSetItem::Ascii(class_ascii);
    let mut writer = Printer { _priv: () };
    let mut visitor = Writer { printer: &mut writer, wtr: String::new() };
    let _ = visitor.visit_class_set_item_post(&ast_item);
}

