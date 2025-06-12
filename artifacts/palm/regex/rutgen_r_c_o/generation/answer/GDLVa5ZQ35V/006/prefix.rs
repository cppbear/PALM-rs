// Answer 0

#[test]
fn test_visit_post_empty_ast() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let ast = Ast::Empty(Span::new(0, 0));
    writer.visit_post(&ast);
}

#[test]
fn test_visit_post_flags_ast() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let flags = SetFlags { span: Span::new(0, 1), flags: Flags::new() };
    let ast = Ast::Flags(flags);
    writer.visit_post(&ast);
}

#[test]
fn test_visit_post_literal_ast() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let literal = Literal { span: Span::new(0, 1), kind: LiteralKind::Verbatim, c: 'a' };
    let ast = Ast::Literal(literal);
    writer.visit_post(&ast);
}

#[test]
fn test_visit_post_dot_ast() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let ast = Ast::Dot(Span::new(0, 1));
    writer.visit_post(&ast);
}

#[test]
fn test_visit_post_assertion_ast() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let assertion = Assertion { span: Span::new(0, 1), kind: AssertionKind::StartLine };
    let ast = Ast::Assertion(assertion);
    writer.visit_post(&ast);
}

#[test]
fn test_visit_post_class_unicode_ast() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let class_unicode = ClassUnicode { span: Span::new(0, 1), negated: false, kind: ClassUnicodeKind::OneLetter('a') };
    let ast = Ast::Class(Class::Unicode(class_unicode));
    writer.visit_post(&ast);
}

#[test]
fn test_visit_post_class_bracketed_ast() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let class_bracketed = ClassBracketed { span: Span::new(0, 1), negated: false, kind: ClassSet::Normal };
    let ast = Ast::Class(Class::Bracketed(class_bracketed));
    writer.visit_post(&ast);
}

#[test]
fn test_visit_post_class_perl_ast() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let class_perl = ClassPerl { span: Span::new(0, 1), kind: ClassPerlKind::Digit, negated: false };
    let ast = Ast::Class(Class::Perl(class_perl));
    writer.visit_post(&ast);
}

#[test]
fn test_visit_post_repetition_ast() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let repetition = Repetition { span: Span::new(0, 1), op: RepetitionOp::ZeroOrMore, greedy: true, ast: Box::new(Ast::Empty(Span::new(0, 0))) };
    let ast = Ast::Repetition(repetition);
    writer.visit_post(&ast);
}

#[test]
fn test_visit_post_group_ast() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let group = Group { span: Span::new(0, 1), kind: GroupKind::Normal, ast: Box::new(Ast::Empty(Span::new(0, 0))) };
    let ast = Ast::Group(group);
    writer.visit_post(&ast);
}

#[test]
fn test_visit_post_alternation_ast() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let ast = Ast::Alternation(vec![]);
    writer.visit_post(&ast);
}

#[test]
fn test_visit_post_concat_ast() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let ast = Ast::Concat(vec![]);
    writer.visit_post(&ast);
}

