// Answer 0

#[test]
fn test_visit_class_set_item_pre_negated() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let ast = ast::ClassSetItem::Bracketed(Box::new(ast::ClassBracketed {
        span: Span::new(0, 1),
        negated: true,
        kind: ClassSet::Normal,
    }));
    writer.visit_class_set_item_pre(&ast).unwrap();
}

#[test]
fn test_visit_class_set_item_pre_not_negated_intersection() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let ast = ast::ClassSetItem::Bracketed(Box::new(ast::ClassBracketed {
        span: Span::new(1, 2),
        negated: false,
        kind: ClassSet::Intersection,
    }));
    writer.visit_class_set_item_pre(&ast).unwrap();
}

#[test]
fn test_visit_class_set_item_pre_not_negated_union() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let ast = ast::ClassSetItem::Bracketed(Box::new(ast::ClassBracketed {
        span: Span::new(2, 3),
        negated: false,
        kind: ClassSet::Union,
    }));
    writer.visit_class_set_item_pre(&ast).unwrap();
}

