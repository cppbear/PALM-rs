// Answer 0

#[test]
fn test_hir_unicode_class_unicode_not_allowed() {
    struct TestStruct;

    impl TestStruct {
        fn flags(&self) -> Flags {
            Flags { unicode_enabled: false }
        }
        
        fn error(&self, _span: Span, kind: ErrorKind) -> Result<hir::ClassUnicode> {
            Err(kind)
        }
    }

    struct Flags {
        unicode_enabled: bool,
    }

    struct Span;

    enum ErrorKind {
        UnicodeNotAllowed,
    }

    let test_struct = TestStruct;
    let ast_class = ast::ClassUnicode { 
        span: Span, 
        kind: ast::ClassUnicodeKind::OneLetter('a'),  // A generic input for the test 
        negated: false 
    };

    let result = test_struct.hir_unicode_class(&ast_class);
    assert!(result.is_err()); // should return an error
    if let Err(error) = result {
        assert_eq!(error, ErrorKind::UnicodeNotAllowed);
    }
}

