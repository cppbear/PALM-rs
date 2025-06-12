// Answer 0

#[test]
fn test_hir_perl_byte_class_digit_non_negated() {
    struct TestFlags {
        unicode: bool,
    }

    impl TestFlags {
        fn unicode(&self) -> bool {
            self.unicode
        }
    }

    struct TestStruct {
        flags: TestFlags,
    }

    impl TestStruct {
        fn flags(&self) -> &TestFlags {
            &self.flags
        }

        fn hir_perl_byte_class(&self, ast_class: &ast::ClassPerl) -> hir::ClassBytes {
            use ast::ClassPerlKind::*;

            assert!(!self.flags().unicode());
            let mut class = match ast_class.kind {
                Digit => hir_ascii_class_bytes(&ast::ClassAsciiKind::Digit),
                Space => hir_ascii_class_bytes(&ast::ClassAsciiKind::Space),
                Word => hir_ascii_class_bytes(&ast::ClassAsciiKind::Word),
            };
            if ast_class.negated {
                class.negate();
            }
            class
        }
    }

    struct MockAstClass {
        kind: ast::ClassPerlKind,
        negated: bool,
    }

    let test_struct = TestStruct {
        flags: TestFlags { unicode: false },
    };

    let ast_class = MockAstClass {
        kind: ast::ClassPerlKind::Digit,
        negated: false,
    };

    let result = test_struct.hir_perl_byte_class(&ast_class);
    // Asserting the expected result based on the kind being Digit and not negated.
    assert_eq!(result, hir_ascii_class_bytes(&ast::ClassAsciiKind::Digit));
}

