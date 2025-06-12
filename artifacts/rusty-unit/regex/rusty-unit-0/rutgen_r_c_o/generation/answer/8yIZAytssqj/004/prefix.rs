// Answer 0

#[test]
fn test_visit_pre_class_bracketed_negated() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut output };

    let ast = Ast::Class(ast::Class::Bracketed(ast::ClassBracketed {
        span: Span::default(),
        negated: true,
        kind: ClassSet::Union(vec![]), // empty class set for edge case
    }));

    writer.visit_pre(&ast);
}

#[test]
fn test_visit_pre_class_bracketed_non_negated() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut output };

    let ast = Ast::Class(ast::Class::Bracketed(ast::ClassBracketed {
        span: Span::default(),
        negated: false,
        kind: ClassSet::Union(vec!['a', 'b', 'c']), // normal class set
    }));

    writer.visit_pre(&ast);
}

#[test]
fn test_visit_pre_class_bracketed_large_range() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut output };

    let ast = Ast::Class(ast::Class::Bracketed(ast::ClassBracketed {
        span: Span::default(),
        negated: false,
        kind: ClassSet::Range('a', 'z'), // character range
    }));

    writer.visit_pre(&ast);
}

#[test]
fn test_visit_pre_class_bracketed_empty_set() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut output };

    let ast = Ast::Class(ast::Class::Bracketed(ast::ClassBracketed {
        span: Span::default(),
        negated: true,
        kind: ClassSet::Union(vec![]), // empty negated set
    }));

    writer.visit_pre(&ast);
}

#[test]
fn test_visit_pre_class_bracketed_single_char() {
    let mut output = String::new();
    let printer = &mut Printer { _priv: () };
    let mut writer = Writer { printer, wtr: &mut output };

    let ast = Ast::Class(ast::Class::Bracketed(ast::ClassBracketed {
        span: Span::default(),
        negated: false,
        kind: ClassSet::Union(vec!['x']), // single character class
    }));

    writer.visit_pre(&ast);
}

