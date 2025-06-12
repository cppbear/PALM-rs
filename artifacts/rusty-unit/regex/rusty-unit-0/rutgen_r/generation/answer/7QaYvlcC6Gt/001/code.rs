// Answer 0

#[test]
#[should_panic]
fn test_hir_perl_byte_class_unicode_flag() {
    struct FakeFlags {
        unicode: bool,
    }

    impl FakeFlags {
        fn unicode(&self) -> bool {
            self.unicode
        }
    }

    struct FakeSelf {
        flags: FakeFlags,
    }

    impl FakeSelf {
        fn flags(&self) -> &FakeFlags {
            &self.flags
        }

        fn hir_perl_byte_class(&self, ast_class: &ast::ClassPerl) -> hir::ClassBytes {
            // Function implementation as provided
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

    struct AstClassPerl {
        kind: ast::ClassPerlKind,
        negated: bool,
    }

    let instance = FakeSelf {
        flags: FakeFlags { unicode: true },
    };
    
    let ast_class_digit = AstClassPerl {
        kind: ast::ClassPerlKind::Digit,
        negated: false,
    };

    let _ = instance.hir_perl_byte_class(&ast_class_digit);
}

