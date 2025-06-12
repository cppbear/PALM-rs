// Answer 0

#[test]
fn test_visit_char_valid_a() {
    let visitor = CharVisitor;
    let result = visitor.visit_char('a');
}

#[test]
fn test_visit_char_valid_b() {
    let visitor = CharVisitor;
    let result = visitor.visit_char('b');
}

#[test]
fn test_visit_char_valid_z() {
    let visitor = CharVisitor;
    let result = visitor.visit_char('z');
}

#[test]
fn test_visit_char_valid_space() {
    let visitor = CharVisitor;
    let result = visitor.visit_char(' ');
}

#[test]
fn test_visit_char_valid_upper_case() {
    let visitor = CharVisitor;
    let result = visitor.visit_char('A');
}

#[test]
fn test_visit_char_valid_special_char() {
    let visitor = CharVisitor;
    let result = visitor.visit_char('!');
}

