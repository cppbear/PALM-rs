// Answer 0

#[test]
fn test_class_induct_literal_char_0() {
    let literal = ast::ClassSetItem::Literal(ast::Literal::from_char('\0'));
    let induct = ClassInduct::Item(&literal);
    let mut buffer = String::new();
    let mut formatter = fmt::Formatter::new(&mut buffer);
    induct.fmt(&mut formatter).unwrap();
}

#[test]
fn test_class_induct_literal_char_1() {
    let literal = ast::ClassSetItem::Literal(ast::Literal::from_char('\u{1}'));
    let induct = ClassInduct::Item(&literal);
    let mut buffer = String::new();
    let mut formatter = fmt::Formatter::new(&mut buffer);
    induct.fmt(&mut formatter).unwrap();
}

#[test]
fn test_class_induct_literal_char_127() {
    let literal = ast::ClassSetItem::Literal(ast::Literal::from_char('\u{7F}'));
    let induct = ClassInduct::Item(&literal);
    let mut buffer = String::new();
    let mut formatter = fmt::Formatter::new(&mut buffer);
    induct.fmt(&mut formatter).unwrap();
}

#[test]
fn test_class_induct_literal_char_255() {
    let literal = ast::ClassSetItem::Literal(ast::Literal::from_char('\u{FF}'));
    let induct = ClassInduct::Item(&literal);
    let mut buffer = String::new();
    let mut formatter = fmt::Formatter::new(&mut buffer);
    induct.fmt(&mut formatter).unwrap();
}

#[test]
fn test_class_induct_literal_char_32() {
    let literal = ast::ClassSetItem::Literal(ast::Literal::from_char(' '));
    let induct = ClassInduct::Item(&literal);
    let mut buffer = String::new();
    let mut formatter = fmt::Formatter::new(&mut buffer);
    induct.fmt(&mut formatter).unwrap();
}

#[test]
fn test_class_induct_literal_char_a() {
    let literal = ast::ClassSetItem::Literal(ast::Literal::from_char('a'));
    let induct = ClassInduct::Item(&literal);
    let mut buffer = String::new();
    let mut formatter = fmt::Formatter::new(&mut buffer);
    induct.fmt(&mut formatter).unwrap();
}

#[test]
fn test_class_induct_literal_char_z() {
    let literal = ast::ClassSetItem::Literal(ast::Literal::from_char('z'));
    let induct = ClassInduct::Item(&literal);
    let mut buffer = String::new();
    let mut formatter = fmt::Formatter::new(&mut buffer);
    induct.fmt(&mut formatter).unwrap();
}

