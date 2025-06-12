// Answer 0

#[test]
fn test_span_bracketed() {
    struct Position(usize);
    #[derive(Clone, Copy, Eq, PartialEq)]
    struct Span {
        start: Position,
        end: Position,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct ClassBracketed {
        span: Span,
        negated: bool,
        kind: String, // Using String as a placeholder for ClassSet
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    enum Class {
        Bracketed(ClassBracketed),
    }

    impl Class {
        pub fn span(&self) -> &Span {
            match *self {
                Class::Bracketed(ref x) => &x.span,
            }
        }
    }

    let position_start = Position(2);
    let position_end = Position(5);
    let span = Span {
        start: position_start,
        end: position_end,
    };
    
    let bracketed_class = Class::Bracketed(ClassBracketed {
        span,
        negated: false,
        kind: "Normal".to_string(),
    });

    let result_span = bracketed_class.span();
    assert_eq!(result_span.start, position_start);
    assert_eq!(result_span.end, position_end);
}

