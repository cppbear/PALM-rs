// Answer 0

#[test]
fn test_class_induct_unicode_valid_1() {
    let unicode_item = ast::ClassSetItem::Unicode(ast::ClassUnicode::UnicodeType::P(Literal::new(r"\pL")));
    let class_induct = ClassInduct::Item(&unicode_item);
    let mut buf = String::new();
    let _ = fmt(&class_induct, &mut buf);
}

#[test]
fn test_class_induct_unicode_valid_2() {
    let unicode_item = ast::ClassSetItem::Unicode(ast::ClassUnicode::UnicodeType::P(Literal::new(r"\p{Greek}")));
    let class_induct = ClassInduct::Item(&unicode_item);
    let mut buf = String::new();
    let _ = fmt(&class_induct, &mut buf);
}

#[should_panic]
fn test_class_induct_unicode_invalid() {
    let unicode_item = ast::ClassSetItem::Unicode(ast::ClassUnicode::UnicodeType::Invalid);
    let class_induct = ClassInduct::Item(&unicode_item);
    let mut buf = String::new();
    let _ = fmt(&class_induct, &mut buf);
}

