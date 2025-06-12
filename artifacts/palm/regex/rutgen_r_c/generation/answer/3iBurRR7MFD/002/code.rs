// Answer 0

#[test]
fn test_class_perl_span() {
    struct Position(usize);
    struct ClassPerlKind;

    let span = Span {
        start: Position(0),
        end: Position(5),
    };
    
    let class_perl = Class::Perl(ClassPerl {
        span,
        kind: ClassPerlKind,
        negated: false,
    });
    
    let result_span = class_perl.span();
    
    assert_eq!(result_span.start.0, 0);
    assert_eq!(result_span.end.0, 5);
}

#[test]
fn test_class_unicode_span() {
    struct ClassUnicodeKind;

    let span = Span {
        start: Position(10),
        end: Position(15),
    };

    let class_unicode = Class::Unicode(ClassUnicode {
        span,
        negated: false,
        kind: ClassUnicodeKind,
    });

    let result_span = class_unicode.span();
    
    assert_eq!(result_span.start.0, 10);
    assert_eq!(result_span.end.0, 15);
}

#[test]
fn test_class_bracketed_span() {
    struct ClassSet;
    
    let span = Span {
        start: Position(20),
        end: Position(25),
    };

    let class_bracketed = Class::Bracketed(ClassBracketed {
        span,
        negated: false,
        kind: ClassSet,
    });

    let result_span = class_bracketed.span();
    
    assert_eq!(result_span.start.0, 20);
    assert_eq!(result_span.end.0, 25);
}

