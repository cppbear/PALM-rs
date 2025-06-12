// Answer 0

#[test]
#[should_panic]
fn test_hir_perl_byte_class_unicode_flag_true() {
    struct TestTranslator {
        flags: Cell<Flags>,
    }
    
    impl TestTranslator {
        fn new(flags: Flags) -> Self {
            Self {
                flags: Cell::new(flags),
            }
        }
    }
    
    let translator = TestTranslator::new(Flags {
        unicode: Some(true),
        ..Flags::default()
    });
    
    let test_class = ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Digit,
        negated: false,
    };
    
    let _ = translator.hir_perl_byte_class(&test_class);
}

#[test]
fn test_hir_perl_byte_class_digit_non_negated() {
    struct TestTranslator {
        flags: Cell<Flags>,
    }
    
    impl TestTranslator {
        fn new(flags: Flags) -> Self {
            Self {
                flags: Cell::new(flags),
            }
        }
        
        fn flags(&self) -> Flags {
            self.flags.get()
        }
        
        fn hir_perl_byte_class(&self, ast_class: &ast::ClassPerl) -> ClassBytes {
            assert!(!self.flags().unicode());
            let mut class = match ast_class.kind {
                ast::ClassPerlKind::Digit => hir_ascii_class_bytes(&ast::ClassAsciiKind::Digit),
                ast::ClassPerlKind::Space => hir_ascii_class_bytes(&ast::ClassAsciiKind::Space),
                ast::ClassPerlKind::Word => hir_ascii_class_bytes(&ast::ClassAsciiKind::Word),
            };
            if ast_class.negated {
                class.negate();
            }
            class
        }
    }
    
    let translator = TestTranslator::new(Flags {
        unicode: Some(false),
        ..Flags::default()
    });
    
    let test_class = ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Digit,
        negated: false,
    };
    
    let result = translator.hir_perl_byte_class(&test_class);
    assert!(result.is_all_ascii());
}

#[test]
fn test_hir_perl_byte_class_space_negated() {
    struct TestTranslator {
        flags: Cell<Flags>,
    }
    
    impl TestTranslator {
        fn new(flags: Flags) -> Self {
            Self {
                flags: Cell::new(flags),
            }
        }
        
        fn flags(&self) -> Flags {
            self.flags.get()
        }
        
        fn hir_perl_byte_class(&self, ast_class: &ast::ClassPerl) -> ClassBytes {
            assert!(!self.flags().unicode());
            let mut class = match ast_class.kind {
                ast::ClassPerlKind::Digit => hir_ascii_class_bytes(&ast::ClassAsciiKind::Digit),
                ast::ClassPerlKind::Space => hir_ascii_class_bytes(&ast::ClassAsciiKind::Space),
                ast::ClassPerlKind::Word => hir_ascii_class_bytes(&ast::ClassAsciiKind::Word),
            };
            if ast_class.negated {
                class.negate();
            }
            class
        }
    }
    
    let translator = TestTranslator::new(Flags {
        unicode: Some(false),
        ..Flags::default()
    });
    
    let test_class = ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Space,
        negated: true,
    };
    
    let result = translator.hir_perl_byte_class(&test_class);
    assert!(!result.is_all_ascii());
}

#[test]
fn test_hir_perl_byte_class_word_non_negated() {
    struct TestTranslator {
        flags: Cell<Flags>,
    }
    
    impl TestTranslator {
        fn new(flags: Flags) -> Self {
            Self {
                flags: Cell::new(flags),
            }
        }
        
        fn flags(&self) -> Flags {
            self.flags.get()
        }
        
        fn hir_perl_byte_class(&self, ast_class: &ast::ClassPerl) -> ClassBytes {
            assert!(!self.flags().unicode());
            let mut class = match ast_class.kind {
                ast::ClassPerlKind::Digit => hir_ascii_class_bytes(&ast::ClassAsciiKind::Digit),
                ast::ClassPerlKind::Space => hir_ascii_class_bytes(&ast::ClassAsciiKind::Space),
                ast::ClassPerlKind::Word => hir_ascii_class_bytes(&ast::ClassAsciiKind::Word),
            };
            if ast_class.negated {
                class.negate();
            }
            class
        }
    }
    
    let translator = TestTranslator::new(Flags {
        unicode: Some(false),
        ..Flags::default()
    });
    
    let test_class = ast::ClassPerl {
        span: Span::default(),
        kind: ast::ClassPerlKind::Word,
        negated: false,
    };
    
    let result = translator.hir_perl_byte_class(&test_class);
    assert!(result.is_all_ascii());
}

