// Answer 0

#[test]
fn test_fmt_class_bracketed_post_with_normal_set() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let ast = ast::ClassBracketed { span: Default::default(), negated: false, kind: ast::ClassSet::Normal };
    writer.fmt_class_bracketed_post(&ast);
}

#[test]
fn test_fmt_class_bracketed_post_with_negated_set() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let ast = ast::ClassBracketed { span: Default::default(), negated: true, kind: ast::ClassSet::Normal };
    writer.fmt_class_bracketed_post(&ast);
}

#[test]
fn test_fmt_class_bracketed_post_with_empty_set() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let ast = ast::ClassBracketed { span: Default::default(), negated: false, kind: ast::ClassSet::ResultOfSetOperations };
    writer.fmt_class_bracketed_post(&ast);
}

#[test]
fn test_fmt_class_bracketed_post_with_large_set() {
    let mut output = String::new();
    let printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: &mut output };
    let ast = ast::ClassBracketed { span: Default::default(), negated: true, kind: ast::ClassSet::ResultOfSetOperations };
    writer.fmt_class_bracketed_post(&ast);
}

