// Answer 0

#[test]
fn test_visit_post_with_alternation() {
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let ast = Ast::Alternation(vec![
        Ast::Group(Group {
            span: Span { start: 0, end: 1 },
            kind: GroupKind::NonCapturing,
            ast: Box::new(Ast::Empty(Span { start: 0, end: 1 })),
        }),
        Ast::Class(ast::Class::Unicode(ClassUnicode {
            span: Span { start: 1, end: 2 },
            negated: false,
            kind: ClassUnicodeKind::OneLetter('a'),
        })),
    ]);
    
    translator.push(HirFrame::Expr(Hir::group(Group {
        span: Span { start: 0, end: 1 },
        kind: GroupKind::NonCapturing,
        ast: Box::new(Ast::Empty(Span { start: 0, end: 1 })),
    })));
    translator.push(HirFrame::Expr(Hir::class(Class::Unicode(ClassUnicode {
        span: Span { start: 1, end: 2 },
        negated: false,
        kind: ClassUnicodeKind::OneLetter('a'),
    }))));
    
    let result = translator.visit_post(&ast);
}

