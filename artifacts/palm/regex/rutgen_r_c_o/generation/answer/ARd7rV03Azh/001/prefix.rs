// Answer 0

#[test]
fn test_print_empty_ast() {
    let mut printer = Printer::new();
    let ast = Ast::Empty(Span::default());
    let mut output = String::new();
    printer.print(&ast, &mut output);
}

#[test]
fn test_print_flags_ast() {
    let mut printer = Printer::new();
    let ast = Ast::Flags(SetFlags::new(vec!['i', 's']));
    let mut output = String::new();
    printer.print(&ast, &mut output);
}

#[test]
fn test_print_literal_ast() {
    let mut printer = Printer::new();
    let ast = Ast::Literal(Literal::new('a')); // ASCII printable character
    let mut output = String::new();
    printer.print(&ast, &mut output);
}

#[test]
fn test_print_dot_ast() {
    let mut printer = Printer::new();
    let ast = Ast::Dot(Span::default());
    let mut output = String::new();
    printer.print(&ast, &mut output);
}

#[test]
fn test_print_assertion_ast() {
    let mut printer = Printer::new();
    let ast = Ast::Assertion(Assertion::new());
    let mut output = String::new();
    printer.print(&ast, &mut output);
}

#[test]
fn test_print_class_ast() {
    let mut printer = Printer::new();
    let ast = Ast::Class(Class::new(vec!['a', 'b', 'c'])); // Valid character class
    let mut output = String::new();
    printer.print(&ast, &mut output);
}

#[test]
fn test_print_repetition_ast() {
    let mut printer = Printer::new();
    let ast = Ast::Repetition(Repetition::new(0, 5)); // Repeat 0 to 5 times
    let mut output = String::new();
    printer.print(&ast, &mut output);
}

#[test]
fn test_print_group_ast() {
    let mut printer = Printer::new();
    let ast = Ast::Group(Group::new(vec![Ast::Literal(Literal::new('a'))])); // 1 level deep
    let mut output = String::new();
    printer.print(&ast, &mut output);
}

#[test]
fn test_print_alternation_ast() {
    let mut printer = Printer::new();
    let ast = Ast::Alternation(Alternation::new(vec![Ast::Literal(Literal::new('a')), Ast::Literal(Literal::new('b'))])); // 2 alternatives
    let mut output = String::new();
    printer.print(&ast, &mut output);
}

#[test]
fn test_print_concat_ast() {
    let mut printer = Printer::new();
    let ast = Ast::Concat(Concat::new(vec![Ast::Literal(Literal::new('a')), Ast::Literal(Literal::new('b'))])); // 2 expressions
    let mut output = String::new();
    printer.print(&ast, &mut output);
}

#[test]
fn test_print_nested_group_ast() {
    let mut printer = Printer::new();
    let ast = Ast::Group(Group::new(vec![
        Ast::Group(Group::new(vec![Ast::Literal(Literal::new('a'))])), // 2 levels deep
    ]));
    let mut output = String::new();
    printer.print(&ast, &mut output);
}

#[test]
fn test_print_complex_ast() {
    let mut printer = Printer::new();
    let ast = Ast::Concat(Concat::new(vec![
        Ast::Class(Class::new(vec!['d'])),
        Ast::Repetition(Repetition::new(1, 3)),
        Ast::Alternation(Alternation::new(vec![Ast::Literal(Literal::new('c')), Ast::Dot(Span::default())])) // Complex nested structure
    ]));
    let mut output = String::new();
    printer.print(&ast, &mut output);
}

