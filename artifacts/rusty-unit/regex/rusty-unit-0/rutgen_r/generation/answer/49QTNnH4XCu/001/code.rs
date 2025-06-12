// Answer 0

#[test]
fn test_error_creation() {
    struct TestRegexParser {
        pattern_str: String,
    }

    impl TestRegexParser {
        fn pattern(&self) -> &str {
            &self.pattern_str
        }

        fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
            ast::Error {
                kind: kind,
                pattern: self.pattern().to_string(),
                span: span,
            }
        }
    }

    struct Span {
        start: usize,
        end: usize,
    }

    mod ast {
        pub struct Error {
            pub kind: ErrorKind,
            pub pattern: String,
            pub span: super::Span,
        }

        #[derive(Debug, PartialEq)]
        pub enum ErrorKind {
            InvalidCharacter,
            UnclosedGroup,
        }
    }

    let parser = TestRegexParser {
        pattern_str: String::from("a*b+c?"),
    };

    // Test a valid error creation
    let span = Span { start: 0, end: 5 };
    let kind = ast::ErrorKind::InvalidCharacter;
    let error = parser.error(span, kind.clone());
    assert_eq!(error.kind, kind);
    assert_eq!(error.pattern, "a*b+c?");
    assert_eq!(error.span.start, 0);
    assert_eq!(error.span.end, 5);

    // Test another error creation
    let span = Span { start: 6, end: 10 };
    let kind = ast::ErrorKind::UnclosedGroup;
    let error = parser.error(span, kind.clone());
    assert_eq!(error.kind, kind);
    assert_eq!(error.pattern, "a*b+c?");
    assert_eq!(error.span.start, 6);
    assert_eq!(error.span.end, 10);
}

