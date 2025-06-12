// Answer 0

#[test]
fn test_hir_perl_unicode_class_space_negated() {
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

        fn hir_perl_unicode_class(
            &self,
            ast_class: &ast::ClassPerl,
        ) -> hir::ClassUnicode {
            use ast::ClassPerlKind::*;
            use unicode_tables::perl_word::PERL_WORD;

            assert!(self.flags().unicode());
            let mut class = match ast_class.kind {
                Digit => {
                    let query = ClassQuery::Binary("Decimal_Number");
                    unicode::class(query).unwrap()
                }
                Space => {
                    let query = ClassQuery::Binary("Whitespace");
                    unicode::class(query).unwrap()
                }
                Word => unicode::hir_class(PERL_WORD),
            };
            if ast_class.negated {
                class.negate();
            }
            class
        }
    }

    struct AstClass {
        kind: ast::ClassPerlKind,
        negated: bool,
    }

    let mock_self = MockSelf {
        flags: MockFlags { unicode: true },
    };

    let ast_class = AstClass {
        kind: ast::ClassPerlKind::Space,
        negated: true,
    };

    let result = mock_self.hir_perl_unicode_class(&ast_class);
    // Here, place your assertions on 'result' to verify its correctness based on the expectations
}

