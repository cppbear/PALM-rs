// Answer 0

#[test]
fn test_visit_class_set_item_post_literal_a() {
    let mut writer = Printer { _priv: () };
    let mut ast_item = ast::ClassSetItem::Literal(ast::Literal {
        span: Span { start: 0, end: 1 },
        kind: ast::LiteralKind::Verbatim,
        c: 'a',
    });
    let mut visitor = Writer { printer: &mut writer, wtr: String::new() };
    visitor.visit_class_set_item_post(&ast_item).unwrap();
}

#[test]
fn test_visit_class_set_item_post_literal_z() {
    let mut writer = Printer { _priv: () };
    let mut ast_item = ast::ClassSetItem::Literal(ast::Literal {
        span: Span { start: 0, end: 1 },
        kind: ast::LiteralKind::Verbatim,
        c: 'z',
    });
    let mut visitor = Writer { printer: &mut writer, wtr: String::new() };
    visitor.visit_class_set_item_post(&ast_item).unwrap();
}

#[test]
fn test_visit_class_set_item_post_literal_range() {
    let mut writer = Printer { _priv: () };
    let start_literal = ast::Literal {
        span: Span { start: 0, end: 1 },
        kind: ast::LiteralKind::Verbatim,
        c: 'a',
    };
    let end_literal = ast::Literal {
        span: Span { start: 0, end: 1 },
        kind: ast::LiteralKind::Verbatim,
        c: 'z',
    };
    let mut ast_item = ast::ClassSetItem::Range(ast::ClassSetRange {
        span: Span { start: 0, end: 3 },
        start: start_literal,
        end: end_literal,
    });
    let mut visitor = Writer { printer: &mut writer, wtr: String::new() };
    visitor.visit_class_set_item_post(&ast_item).unwrap();
}

