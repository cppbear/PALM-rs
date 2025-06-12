// Answer 0

#[test]
fn test_visit_class_set_item_post_perl_digit_negated() {
    let mut writer = String::new();
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    let perl_class = ast::ClassPerl { span: Span::default(), kind: ast::ClassPerlKind::Digit, negated: true };
    let ast_item = ast::ClassSetItem::Perl(perl_class);
    let _ = visitor.visit_class_set_item_post(&ast_item);
}

#[test]
fn test_visit_class_set_item_post_perl_digit_not_negated() {
    let mut writer = String::new();
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    let perl_class = ast::ClassPerl { span: Span::default(), kind: ast::ClassPerlKind::Digit, negated: false };
    let ast_item = ast::ClassSetItem::Perl(perl_class);
    let _ = visitor.visit_class_set_item_post(&ast_item);
}

#[test]
fn test_visit_class_set_item_post_perl_space_negated() {
    let mut writer = String::new();
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    let perl_class = ast::ClassPerl { span: Span::default(), kind: ast::ClassPerlKind::Space, negated: true };
    let ast_item = ast::ClassSetItem::Perl(perl_class);
    let _ = visitor.visit_class_set_item_post(&ast_item);
}

#[test]
fn test_visit_class_set_item_post_perl_space_not_negated() {
    let mut writer = String::new();
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    let perl_class = ast::ClassPerl { span: Span::default(), kind: ast::ClassPerlKind::Space, negated: false };
    let ast_item = ast::ClassSetItem::Perl(perl_class);
    let _ = visitor.visit_class_set_item_post(&ast_item);
}

#[test]
fn test_visit_class_set_item_post_perl_word_negated() {
    let mut writer = String::new();
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    let perl_class = ast::ClassPerl { span: Span::default(), kind: ast::ClassPerlKind::Word, negated: true };
    let ast_item = ast::ClassSetItem::Perl(perl_class);
    let _ = visitor.visit_class_set_item_post(&ast_item);
}

#[test]
fn test_visit_class_set_item_post_perl_word_not_negated() {
    let mut writer = String::new();
    let mut visitor = Writer { printer: &mut Printer { _priv: () }, wtr: &mut writer };
    let perl_class = ast::ClassPerl { span: Span::default(), kind: ast::ClassPerlKind::Word, negated: false };
    let ast_item = ast::ClassSetItem::Perl(perl_class);
    let _ = visitor.visit_class_set_item_post(&ast_item);
}

