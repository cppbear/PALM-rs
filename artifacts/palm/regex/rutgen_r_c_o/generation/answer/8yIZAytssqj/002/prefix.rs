// Answer 0

#[test]
fn test_visit_pre_group_capture_index() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let ast = Ast::Group(Group {
        span: Span::default(), 
        kind: GroupKind::CaptureIndex(0), 
        ast: Box::new(Ast::Empty(Span::default()))
    });
    writer.visit_pre(&ast).unwrap();
}

#[test]
fn test_visit_pre_group_capture_name() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let ast = Ast::Group(Group {
        span: Span::default(), 
        kind: GroupKind::CaptureName(Name { name: String::from("group_name") }), 
        ast: Box::new(Ast::Empty(Span::default()))
    });
    writer.visit_pre(&ast).unwrap();
}

#[test]
fn test_visit_pre_group_non_capturing() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let ast = Ast::Group(Group {
        span: Span::default(), 
        kind: GroupKind::NonCapturing(Vec::new()), 
        ast: Box::new(Ast::Empty(Span::default()))
    });
    writer.visit_pre(&ast).unwrap();
}

#[test]
fn test_visit_pre_class_bracketed_negated() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let ast = Ast::Class(ast::Class::Bracketed(ClassBracketed {
        span: Span::default(), 
        negated: true, 
        kind: ClassSet::Union(Vec::new())
    }));
    writer.visit_pre(&ast).unwrap();
}

#[test]
fn test_visit_pre_class_bracketed_non_negated() {
    let mut printer = Printer { _priv: () };
    let mut writer = Writer { printer: &mut printer, wtr: String::new() };
    let ast = Ast::Class(ast::Class::Bracketed(ClassBracketed {
        span: Span::default(), 
        negated: false, 
        kind: ClassSet::Union(Vec::new())
    }));
    writer.visit_pre(&ast).unwrap();
}

