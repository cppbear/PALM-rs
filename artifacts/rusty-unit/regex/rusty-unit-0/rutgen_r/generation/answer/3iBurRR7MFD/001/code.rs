// Answer 0

#[test]
fn test_span_bracketed() {
    struct Span {
        start: usize,
        end: usize,
    }

    struct Bracketed {
        span: Span,
    }

    enum Class {
        Perl(OtherStruct), // OtherStruct will not be defined as it is not used
        Unicode(OtherStruct), // OtherStruct will not be defined as it is not used
        Bracketed(Bracketed),
    }

    impl Class {
        pub fn span(&self) -> &Span {
            match *self {
                Class::Perl(ref _x) => & _x.span, // These match arms are not detailed as they are not used for this test
                Class::Unicode(ref _x) => & _x.span,
                Class::Bracketed(ref x) => &x.span,
            }
        }
    }

    let bracketed_instance = Bracketed {
        span: Span { start: 0, end: 5 },
    };

    let class_instance = Class::Bracketed(bracketed_instance);
    let span = class_instance.span();
    
    assert_eq!(span.start, 0);
    assert_eq!(span.end, 5);
}

