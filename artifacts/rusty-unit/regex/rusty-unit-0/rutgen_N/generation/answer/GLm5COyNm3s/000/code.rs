// Answer 0

#[test]
fn test_span_literal() {
    struct Literal {
        span: Span,
    }
    
    struct Primitive {
        variant: PrimitiveVariant,
    }

    enum PrimitiveVariant {
        Literal(Literal),
    }

    impl Primitive {
        fn span(&self) -> &Span {
            match &self.variant {
                PrimitiveVariant::Literal(ref x) => &x.span,
            }
        }
    }

    let span = Span { /* initialize with appropriate values */ };
    let primitive = Primitive {
        variant: PrimitiveVariant::Literal(Literal { span }),
    };

    assert_eq!(primitive.span(), &span);
}

#[test]
fn test_span_assertion() {
    struct Assertion {
        span: Span,
    }
    
    struct Primitive {
        variant: PrimitiveVariant,
    }

    enum PrimitiveVariant {
        Assertion(Assertion),
    }

    impl Primitive {
        fn span(&self) -> &Span {
            match &self.variant {
                PrimitiveVariant::Assertion(ref x) => &x.span,
            }
        }
    }

    let span = Span { /* initialize with appropriate values */ };
    let primitive = Primitive {
        variant: PrimitiveVariant::Assertion(Assertion { span }),
    };

    assert_eq!(primitive.span(), &span);
}

#[test]
fn test_span_dot() {
    struct Primitive {
        variant: PrimitiveVariant,
    }

    enum PrimitiveVariant {
        Dot(Span),
    }

    impl Primitive {
        fn span(&self) -> &Span {
            match &self.variant {
                PrimitiveVariant::Dot(ref span) => span,
            }
        }
    }

    let span = Span { /* initialize with appropriate values */ };
    let primitive = Primitive {
        variant: PrimitiveVariant::Dot(span),
    };

    assert_eq!(primitive.span(), &span);
}

#[test]
fn test_span_perl() {
    struct Perl {
        span: Span,
    }
    
    struct Primitive {
        variant: PrimitiveVariant,
    }

    enum PrimitiveVariant {
        Perl(Perl),
    }

    impl Primitive {
        fn span(&self) -> &Span {
            match &self.variant {
                PrimitiveVariant::Perl(ref x) => &x.span,
            }
        }
    }

    let span = Span { /* initialize with appropriate values */ };
    let primitive = Primitive {
        variant: PrimitiveVariant::Perl(Perl { span }),
    };

    assert_eq!(primitive.span(), &span);
}

#[test]
fn test_span_unicode() {
    struct Unicode {
        span: Span,
    }
    
    struct Primitive {
        variant: PrimitiveVariant,
    }

    enum PrimitiveVariant {
        Unicode(Unicode),
    }

    impl Primitive {
        fn span(&self) -> &Span {
            match &self.variant {
                PrimitiveVariant::Unicode(ref x) => &x.span,
            }
        }
    }

    let span = Span { /* initialize with appropriate values */ };
    let primitive = Primitive {
        variant: PrimitiveVariant::Unicode(Unicode { span }),
    };

    assert_eq!(primitive.span(), &span);
}

