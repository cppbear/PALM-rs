// Answer 0

#[test]
fn test_visit_post_empty() {
    let translator = Translator { 
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let mut translator_i = TranslatorI::new(&translator, "");
    let ast = Ast::Empty(Span { start: 0, end: 0 });
    translator_i.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_flags() {
    let translator = Translator { 
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { case_insensitive: Some(true), multi_line: None, dot_matches_new_line: None, swap_greed: None, unicode: Some(true)}),
        allow_invalid_utf8: false,
    };
    let mut translator_i = TranslatorI::new(&translator, "");
    let flags = SetFlags { span: Span { start: 0, end: 1 }, flags: Flags { case_insensitive: Some(true), multi_line: None, dot_matches_new_line: None, swap_greed: None, unicode: Some(true)}};
    let ast = Ast::Flags(flags);
    translator_i.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_perl_digit_class() {
    let translator = Translator { 
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(true), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let mut translator_i = TranslatorI::new(&translator, "");
    let perl_class = ClassPerl { span: Span { start: 0, end: 1 }, kind: ClassPerlKind::Digit, negated: false };
    let ast = Ast::Class(ast::Class::Perl(perl_class));
    translator_i.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_perl_space_class() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(true), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let mut translator_i = TranslatorI::new(&translator, "");
    let perl_class = ClassPerl { span: Span { start: 0, end: 1 }, kind: ClassPerlKind::Space, negated: false };
    let ast = Ast::Class(ast::Class::Perl(perl_class));
    translator_i.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_unicode_class() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(true), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let mut translator_i = TranslatorI::new(&translator, "");
    let unicode_class = ClassUnicode { span: Span { start: 0, end: 1 }, negated: false, kind: ClassUnicodeKind::OneLetter('a') };
    let ast = Ast::Class(ast::Class::Unicode(unicode_class));
    translator_i.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_bracketed_class_unicode() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(true), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let mut translator_i = TranslatorI::new(&translator, "");
    let bracketed_class = ClassBracketed {
        span: Span { start: 0, end: 2 },
        negated: false,
        kind: ClassSet::Normal,
    };
    let ast = Ast::Class(ast::Class::Bracketed(bracketed_class));
    translator_i.visit_post(&ast).unwrap();
}

