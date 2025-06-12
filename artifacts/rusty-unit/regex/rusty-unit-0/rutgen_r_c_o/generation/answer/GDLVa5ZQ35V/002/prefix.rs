// Answer 0

#[test]
fn test_visit_post_empty_ast() {
    let mut output = String::new();
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut output };
    let ast = Ast::Empty(Span);
    writer.visit_post(&ast);
}

#[test]
fn test_visit_post_flags_ast() {
    let mut output = String::new();
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut output };
    let flags = SetFlags { span: Span, flags: Flags };
    let ast = Ast::Flags(flags);
    writer.visit_post(&ast);
}

#[test]
fn test_visit_post_literal_ast() {
    let mut output = String::new();
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut output };
    let literal = Literal { span: Span, kind: LiteralKind::Verbatim, c: 'a' };
    let ast = Ast::Literal(literal);
    writer.visit_post(&ast);
}

#[test]
fn test_visit_post_dot_ast() {
    let mut output = String::new();
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut output };
    let ast = Ast::Dot(Span);
    writer.visit_post(&ast);
}

#[test]
fn test_visit_post_assertion_ast() {
    let mut output = String::new();
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut output };
    let assertion = Assertion { span: Span, kind: AssertionKind::StartLine };
    let ast = Ast::Assertion(assertion);
    writer.visit_post(&ast);
}

#[test]
fn test_visit_post_class_perl_ast() {
    let mut output = String::new();
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut output };
    let class_perl = ClassPerl { span: Span, kind: ClassPerlKind::Digit, negated: false };
    let ast = Ast::Class(Class::Perl(class_perl));
    writer.visit_post(&ast);
}

#[test]
fn test_visit_post_class_unicode_ast() {
    let mut output = String::new();
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut output };
    let class_unicode = ClassUnicode { span: Span, negated: false, kind: ClassUnicodeKind::OneLetter('a') };
    let ast = Ast::Class(Class::Unicode(class_unicode));
    writer.visit_post(&ast);
}

#[test]
fn test_visit_post_class_bracketed_ast() {
    let mut output = String::new();
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut output };
    let class_bracketed = ClassBracketed { span: Span, negated: false, kind: ClassSet::Normal };
    let ast = Ast::Class(Class::Bracketed(class_bracketed));
    writer.visit_post(&ast);
}

#[test]
fn test_visit_post_repetition_ast() {
    let mut output = String::new();
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut output };
    let repetition = Repetition { span: Span, op: RepetitionOp::ZeroOrMore, greedy: true, ast: Box::new(Ast::Empty(Span)) };
    let ast = Ast::Repetition(repetition);
    writer.visit_post(&ast);
}

#[test]
fn test_visit_post_group_ast() {
    let mut output = String::new();
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut output };
    let group = Group { span: Span, kind: GroupKind::Capture, ast: Box::new(Ast::Empty(Span)) };
    let ast = Ast::Group(group);
    writer.visit_post(&ast);
}

#[test]
fn test_visit_post_alternation_ast() {
    let mut output = String::new();
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut output };
    let ast = Ast::Alternation(vec![Ast::Empty(Span), Ast::Empty(Span)]);
    writer.visit_post(&ast);
}

#[test]
fn test_visit_post_concat_ast() {
    let mut output = String::new();
    let mut writer = Writer { printer: &mut Printer { _priv: () }, wtr: &mut output };
    let ast = Ast::Concat(vec![Ast::Empty(Span), Ast::Empty(Span)]);
    writer.visit_post(&ast);
}

