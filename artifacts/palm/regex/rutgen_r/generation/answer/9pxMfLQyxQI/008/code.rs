// Answer 0

#[test]
fn test_from_ast_with_empty_flags() {
    struct TestAstItem {
        kind: ast::FlagsItemKind,
    }

    struct TestAst {
        items: Vec<TestAstItem>,
    }

    let ast = TestAst { items: Vec::new() };
    let flags = from_ast(&ast);
    assert_eq!(flags, Flags::default());
}

#[test]
fn test_from_ast_with_negation() {
    struct TestAstItem {
        kind: ast::FlagsItemKind,
    }

    struct TestAst {
        items: Vec<TestAstItem>,
    }

    let ast = TestAst { 
        items: vec![
            TestAstItem { kind: ast::FlagsItemKind::Negation },
            TestAstItem { kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive) }
        ] 
    };
    let flags = from_ast(&ast);
    assert_eq!(flags.case_insensitive, Some(false));
    assert_eq!(flags.multi_line, None);
}

#[test]
fn test_from_ast_with_case_insensitive_flag() {
    struct TestAstItem {
        kind: ast::FlagsItemKind,
    }

    struct TestAst {
        items: Vec<TestAstItem>,
    }

    let ast = TestAst { 
        items: vec![
            TestAstItem { kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive) }
        ] 
    };
    let flags = from_ast(&ast);
    assert_eq!(flags.case_insensitive, Some(true));
}

#[test]
fn test_from_ast_with_multi_line_flag() {
    struct TestAstItem {
        kind: ast::FlagsItemKind,
    }

    struct TestAst {
        items: Vec<TestAstItem>,
    }

    let ast = TestAst { 
        items: vec![
            TestAstItem { kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine) }
        ] 
    };
    let flags = from_ast(&ast);
    assert_eq!(flags.multi_line, Some(true));
}

#[test]
fn test_from_ast_with_dot_matches_new_line_flag() {
    struct TestAstItem {
        kind: ast::FlagsItemKind,
    }

    struct TestAst {
        items: Vec<TestAstItem>,
    }

    let ast = TestAst { 
        items: vec![
            TestAstItem { kind: ast::FlagsItemKind::Flag(ast::Flag::DotMatchesNewLine) }
        ] 
    };
    let flags = from_ast(&ast);
    assert_eq!(flags.dot_matches_new_line, Some(true));
}

#[test]
fn test_from_ast_with_swap_greed_flag() {
    struct TestAstItem {
        kind: ast::FlagsItemKind,
    }

    struct TestAst {
        items: Vec<TestAstItem>,
    }

    let ast = TestAst { 
        items: vec![
            TestAstItem { kind: ast::FlagsItemKind::Flag(ast::Flag::SwapGreed) }
        ] 
    };
    let flags = from_ast(&ast);
    assert_eq!(flags.swap_greed, Some(true));
}

#[test]
fn test_from_ast_with_unicode_flag() {
    struct TestAstItem {
        kind: ast::FlagsItemKind,
    }

    struct TestAst {
        items: Vec<TestAstItem>,
    }

    let ast = TestAst {
        items: vec![
            TestAstItem { kind: ast::FlagsItemKind::Flag(ast::Flag::Unicode) }
        ]
    };
    let flags = from_ast(&ast);
    assert_eq!(flags.unicode, Some(true));
}

#[test]
fn test_from_ast_with_ignore_whitespace_flag() {
    struct TestAstItem {
        kind: ast::FlagsItemKind,
    }

    struct TestAst {
        items: Vec<TestAstItem>,
    }

    let ast = TestAst { 
        items: vec![
            TestAstItem { kind: ast::FlagsItemKind::Flag(ast::Flag::IgnoreWhitespace) },
        ] 
    };
    let flags = from_ast(&ast);
    assert_eq!(flags.ignore_whitespace, None); // Should remain None as per the function logic
}

