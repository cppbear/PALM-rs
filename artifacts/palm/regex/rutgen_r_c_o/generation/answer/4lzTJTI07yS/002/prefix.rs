// Answer 0

#[test]
fn test_c_with_alternation_multiple_literals() {
    let mut compiler = Compiler::new().size_limit(20);
    let expr = Hir::from(ALTERNATION(vec![
        Hir::from(Literal(hir::Literal::Unicode('a'))),
        Hir::from(Literal(hir::Literal::Unicode('b')))
    ]));
    compiler.c(&expr).unwrap();
}

#[test]
fn test_c_with_alternation_class() {
    let mut compiler = Compiler::new().size_limit(20);
    let expr = Hir::from(ALTERNATION(vec![
        Hir::from(Class(hir::Class::Unicode(UniClass::new(vec![
            (1, 2),
            (3, 4)
        ])))),
        Hir::from(Literal(hir::Literal::Unicode('c')))
    ]));
    compiler.c(&expr).unwrap();
}

#[test]
fn test_c_with_alternation_multiple_groups() {
    let mut compiler = Compiler::new().size_limit(20);
    let expr = Hir::from(ALTERNATION(vec![
        Hir::from(Group::new_non_capturing(Hir::from(Literal(hir::Literal::Unicode('d'))))),
        Hir::from(Group::new_non_capturing(Hir::from(Literal(hir::Literal::Unicode('e')))))
    ]));
    compiler.c(&expr).unwrap();
}

#[test]
fn test_c_with_alternation_reverse() {
    let mut compiler = Compiler::new().size_limit(20).dfa(true);
    let expr = Hir::from(ALTERNATION(vec![
        Hir::from(Literal(hir::Literal::Byte(0b0000_0001))),
        Hir::from(Class(hir::Class::Bytes(ByteClass::new(vec![
            (50, 100)
        ]))))
    ]));
    compiler.c(&expr).unwrap();
}

