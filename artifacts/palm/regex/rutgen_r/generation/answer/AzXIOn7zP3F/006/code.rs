// Answer 0

#[test]
fn test_hir_perl_unicode_class_digit() {
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

        fn hir_perl_unicode_class(&self, ast_class: &ast::ClassPerl) -> hir::ClassUnicode {
            // Original function implementation, for the purpose of testing
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

    struct FakeClassPerl {
        kind: ast::ClassPerlKind,
        negated: bool,
    }

    let test_struct = TestStruct {
        flags: TestFlags { unicode: true },
    };

    let ast_class = FakeClassPerl {
        kind: ast::ClassPerlKind::Digit,
        negated: false,
    };

    let result = test_struct.hir_perl_unicode_class(&ast_class);
    // Additional assertions can be made here to verify the properties of `result`
}

