// Answer 0

#[test]
#[should_panic]
fn test_hir_perl_unicode_class_unicode_flag_false() {
    struct MockSelf {
        unicode_flag: bool,
    }

    impl MockSelf {
        fn flags(&self) -> Flags {
            Flags {
                unicode: self.unicode_flag,
            }
        }
    }

    struct Flags {
        unicode: bool,
    }

    let mock_self = MockSelf { unicode_flag: false };
    
    struct MockAstClass {
        kind: ast::ClassPerlKind,
        negated: bool,
    }

    let ast_class_space = MockAstClass {
        kind: ast::ClassPerlKind::Space,
        negated: false,
    };

    let ast_class_word = MockAstClass {
        kind: ast::ClassPerlKind::Word,
        negated: false,
    };

    let ast_class_digit = MockAstClass {
        kind: ast::ClassPerlKind::Digit,
        negated: true,
    };

    // This will panic due to the unicode flag being false
    let _ = mock_self.hir_perl_unicode_class(&ast_class_space);
    let _ = mock_self.hir_perl_unicode_class(&ast_class_word);
    let _ = mock_self.hir_perl_unicode_class(&ast_class_digit);
}

