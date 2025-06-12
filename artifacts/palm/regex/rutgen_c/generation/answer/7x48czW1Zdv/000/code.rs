// Answer 0

#[test]
fn test_set_flags_case_insensitive() {
    struct DummyTranslator {
        flags: Cell<Flags>,
    }
    
    impl DummyTranslator {
        fn new() -> Self {
            Self { flags: Cell::new(Flags::default()) }
        }
    }

    let trans = DummyTranslator::new();
    let pattern = "test";
    
    let mut translator = TranslatorI::new(&trans, pattern);
    
    let ast_flags = ast::Flags { items: vec![ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive) }] };
    
    let old_flags = translator.set_flags(&ast_flags);
    
    assert_eq!(old_flags.case_insensitive, None);
    assert_eq!(trans.flags.get().case_insensitive, Some(true));
}

#[test]
fn test_set_flags_multi_line() {
    struct DummyTranslator {
        flags: Cell<Flags>,
    }
    
    impl DummyTranslator {
        fn new() -> Self {
            Self { flags: Cell::new(Flags::default()) }
        }
    }

    let trans = DummyTranslator::new();
    let pattern = "test";
    
    let mut translator = TranslatorI::new(&trans, pattern);
    
    let ast_flags = ast::Flags { items: vec![ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine) }] };
    
    let old_flags = translator.set_flags(&ast_flags);
    
    assert_eq!(old_flags.multi_line, None);
    assert_eq!(trans.flags.get().multi_line, Some(true));
}

#[test]
fn test_set_flags_no_changes() {
    struct DummyTranslator {
        flags: Cell<Flags>,
    }
    
    impl DummyTranslator {
        fn new() -> Self {
            let flags = Flags {
                case_insensitive: Some(true),
                multi_line: Some(true),
                ..Flags::default()
            };
            Self { flags: Cell::new(flags) }
        }
    }

    let trans = DummyTranslator::new();
    let pattern = "test";
    
    let mut translator = TranslatorI::new(&trans, pattern);
    
    let ast_flags = ast::Flags { items: vec![] };  // No flags to set
    
    let old_flags = translator.set_flags(&ast_flags);
    
    assert_eq!(old_flags.case_insensitive, Some(true));
    assert_eq!(trans.flags.get().case_insensitive, Some(true));
}

#[test]
fn test_set_flags_negation() {
    struct DummyTranslator {
        flags: Cell<Flags>,
    }
    
    impl DummyTranslator {
        fn new() -> Self {
            let flags = Flags {
                case_insensitive: Some(true),
                ..Flags::default()
            };
            Self { flags: Cell::new(flags) }
        }
    }

    let trans = DummyTranslator::new();
    let pattern = "test";
    
    let mut translator = TranslatorI::new(&trans, pattern);
    
    let ast_flags = ast::Flags { items: vec![
        ast::FlagsItem { kind: ast::FlagsItemKind::Negation },
        ast::FlagsItem { kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive) }
    ]};
    
    let old_flags = translator.set_flags(&ast_flags);
    
    assert_eq!(old_flags.case_insensitive, Some(true));
    assert_eq!(trans.flags.get().case_insensitive, Some(false));
}

