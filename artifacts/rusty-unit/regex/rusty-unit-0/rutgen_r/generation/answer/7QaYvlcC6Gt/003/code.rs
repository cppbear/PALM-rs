// Answer 0

#[test]
fn test_hir_perl_byte_class_word_non_negated() {
    struct MockFlags {
        unicode: bool,
    }

    impl MockFlags {
        fn unicode(&self) -> bool {
            self.unicode
        }
    }

    struct MockSelf {
        flags: MockFlags,
    }

    impl MockSelf {
        fn flags(&self) -> &MockFlags {
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

    let mock_self = MockSelf {
        flags: MockFlags { unicode: false },
    };

    let ast_class = ast::ClassPerl {
        kind: ast::ClassPerlKind::Word,
        negated: false,
    };

    let result = mock_self.hir_perl_byte_class(&ast_class);
    // Assuming `ClassBytes` has some kind of expected value or method to assert correctness
    assert!(result.is_valid()); // Replace with actual validation logic for ClassBytes
}

