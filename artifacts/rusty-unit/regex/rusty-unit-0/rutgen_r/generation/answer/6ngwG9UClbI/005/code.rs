// Answer 0

#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct Class {
    span: Span,
}

#[derive(Debug)]
enum Ast {
    Empty(Span),
    Flags { span: Span },
    Literal { span: Span },
    Dot(Span),
    Assertion { span: Span },
    Class(Class),
    Repetition { span: Span },
    Group { span: Span },
    Alternation { span: Span },
    Concat { span: Span },
}

impl Class {
    fn span(&self) -> &Span {
        &self.span
    }
}

#[test]
fn test_span_class() {
    let span = Span { start: 0, end: 5 };
    let ast = Ast::Class(Class { span });

    if let Ast::Class(ref x) = ast {
        assert_eq!(x.span().start, 0);
        assert_eq!(x.span().end, 5);
    } else {
        panic!("Expected Ast::Class variant");
    }
}

