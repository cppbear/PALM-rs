// Answer 0

fn test_visit_post_empty() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let mut visitor = TranslatorI::new(&translator, "");

    let ast = Ast::Empty(Span { start: 0, end: 0 });
    let result = visitor.visit_post(&ast);
    assert!(result.is_ok());
}

fn test_visit_post_flags() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let mut visitor = TranslatorI::new(&translator, "");

    let ast_flags = ast::Flags {
        span: Span { start: 0, end: 3 },
        flags: Flags { case_insensitive: Some(true), ..Flags::default() },
    };
    let ast = Ast::Flags(ast_flags);
  
    let result = visitor.visit_post(&ast);
    assert!(result.is_ok());
}

fn test_visit_post_literal() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let mut visitor = TranslatorI::new(&translator, "");

    let lit = ast::Literal {
        span: Span { start: 0, end: 1 },
        kind: ast::LiteralKind::Character,
        c: 'a',
    };
    let ast = Ast::Literal(lit);
  
    let result = visitor.visit_post(&ast);
    assert!(result.is_ok());
}

fn test_visit_post_dot() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let mut visitor = TranslatorI::new(&translator, "");

    let ast = Ast::Dot(Span { start: 0, end: 1 });
    let result = visitor.visit_post(&ast);
    assert!(result.is_ok());
}

fn test_visit_post_assertion() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let mut visitor = TranslatorI::new(&translator, "");

    let assertion = ast::Assertion {
        span: Span { start: 0, end: 1 },
        kind: ast::AssertionKind::StartText,
    };
    let ast = Ast::Assertion(assertion);
  
    let result = visitor.visit_post(&ast);
    assert!(result.is_ok());
}

fn test_visit_post_class_perl() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(true), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let mut visitor = TranslatorI::new(&translator, "");

    let class_perl = ast::ClassPerl {
        span: Span { start: 0, end: 1 },
        kind: ast::ClassPerlKind::Digit,
        negated: false,
    };
    let ast = Ast::Class(ast::Class::Perl(class_perl));
  
    let result = visitor.visit_post(&ast);
    assert!(result.is_ok());
}

fn test_visit_post_class_unicode() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(true), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let mut visitor = TranslatorI::new(&translator, "");

    let unicode_class = ast::ClassUnicode {
        span: Span { start: 0, end: 1 },
        kind: ast::ClassUnicodeKind::OneLetter('a'),
        negated: false,
    };
    let ast = Ast::Class(ast::Class::Unicode(unicode_class));
  
    let result = visitor.visit_post(&ast);
    assert!(result.is_ok());
}

fn test_visit_post_class_bracketed_unicode() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags { unicode: Some(true), ..Flags::default() }),
        allow_invalid_utf8: false,
    };
    let mut visitor = TranslatorI::new(&translator, "");

    let ranges = vec![ClassUnicodeRange { start: 'a', end: 'z' }];
    let class_unicode = ClassUnicode::new(ranges);
    let ast = ast::Class::Bracketed(ast::ClassBracketed {
        span: Span { start: 0, end: 3 },
        negated: false,
        kind: ClassSet::Unicode(class_unicode),
    });

    let ast = Ast::Class(ast);
    let result = visitor.visit_post(&ast);
    assert!(result.is_ok());
}

fn test_visit_post_class_bracketed_bytes() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    let mut visitor = TranslatorI::new(&translator, "");

    let ranges = vec![ClassBytesRange { start: 0, end: 255 }];
    let class_bytes = ClassBytes::new(ranges);
    let ast = ast::Class::Bracketed(ast::ClassBracketed {
        span: Span { start: 0, end: 3 },
        negated: false,
        kind: ClassSet::Bytes(class_bytes),
    });

    let ast = Ast::Class(ast);
    let result = visitor.visit_post(&ast);
    assert!(result.is_ok());
}

