// Answer 0

#[test]
fn test_span_unicode() {
    #[derive(Clone, Copy, Eq, PartialEq)]
    struct Position(usize);
    
    #[derive(Clone, Copy, Eq, PartialEq)]
    struct ClassUnicodeRange;

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct IntervalSet<T>(Vec<T>);

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct ClassUnicode {
        span: Span,
        negated: bool,
        kind: ClassUnicodeKind,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Span {
        start: Position,
        end: Position,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    enum ClassUnicodeKind {
        Normal,
        Special,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    enum Class {
        Unicode(ClassUnicode),
        Perl(ClassPerl),
        Bracketed(ClassBracketed),
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct ClassPerl {
        span: Span,
        kind: ClassPerlKind,
        negated: bool,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct ClassBracketed {
        span: Span,
        negated: bool,
        kind: ClassSet,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct ClassPerlKind;

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct ClassSet;

    let test_span = Span {
        start: Position(0),
        end: Position(5),
    };

    let unicode_class = Class::Unicode(ClassUnicode {
        span: test_span,
        negated: false,
        kind: ClassUnicodeKind::Normal,
    });

    assert_eq!(unicode_class.span(), &test_span);
}

