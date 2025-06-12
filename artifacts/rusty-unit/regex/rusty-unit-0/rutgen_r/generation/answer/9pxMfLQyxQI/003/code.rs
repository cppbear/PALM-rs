// Answer 0

fn test_from_ast_case_insensitive() {
    struct TestFlagsItem {
        kind: ast::FlagsItemKind,
    }

    struct TestAst {
        items: Vec<TestFlagsItem>,
    }

    let ast = TestAst {
        items: vec![
            TestFlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
            },
            TestFlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine),
            },
            TestFlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::DotMatchesNewLine),
            },
        ],
    };

    let result = from_ast(&ast);
    assert_eq!(result.case_insensitive, Some(true));  // CaseInsensitive is enabled
    assert_eq!(result.multi_line, Some(true));         // MultiLine is enabled
    assert_eq!(result.dot_matches_new_line, Some(true)); // DotMatchesNewLine is enabled
}

fn test_from_ast_negation() {
    struct TestFlagsItem {
        kind: ast::FlagsItemKind,
    }

    struct TestAst {
        items: Vec<TestFlagsItem>,
    }

    let ast = TestAst {
        items: vec![
            TestFlagsItem {
                kind: ast::FlagsItemKind::Negation,
            },
            TestFlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
            },
            TestFlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::Unicode),
            },
        ],
    };

    let result = from_ast(&ast);
    assert_eq!(result.case_insensitive, Some(false)); // CaseInsensitive is negated
    assert_eq!(result.unicode, Some(false));           // Unicode is negated
}

fn test_from_ast_multiple_flags() {
    struct TestFlagsItem {
        kind: ast::FlagsItemKind,
    }

    struct TestAst {
        items: Vec<TestFlagsItem>,
    }

    let ast = TestAst {
        items: vec![
            TestFlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive),
            },
            TestFlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::MultiLine),
            },
            TestFlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::DotMatchesNewLine),
            },
            TestFlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::SwapGreed),
            },
            TestFlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::Unicode),
            },
        ],
    };
    
    let result = from_ast(&ast);
    assert_eq!(result.case_insensitive, Some(true));
    assert_eq!(result.multi_line, Some(true));
    assert_eq!(result.dot_matches_new_line, Some(true));
    assert_eq!(result.swap_greed, Some(true));
    assert_eq!(result.unicode, Some(true));
}

fn test_from_ast_ignore_whitespace() {
    struct TestFlagsItem {
        kind: ast::FlagsItemKind,
    }

    struct TestAst {
        items: Vec<TestFlagsItem>,
    }

    let ast = TestAst {
        items: vec![
            TestFlagsItem {
                kind: ast::FlagsItemKind::Flag(ast::Flag::IgnoreWhitespace),
            },
        ],
    };

    let result = from_ast(&ast);
    assert_eq!(result.case_insensitive, None); // IgnoreWhitespace does not modify flags
    assert_eq!(result.multi_line, None);
    assert_eq!(result.dot_matches_new_line, None);
    assert_eq!(result.swap_greed, None);
    assert_eq!(result.unicode, None);
}

fn test_from_ast_empty() {
    struct TestFlagsItem {
        kind: ast::FlagsItemKind,
    }

    struct TestAst {
        items: Vec<TestFlagsItem>,
    }

    let ast = TestAst {
        items: vec![],
    };

    let result = from_ast(&ast);
    assert_eq!(result.case_insensitive, None); 
    assert_eq!(result.multi_line, None);
    assert_eq!(result.dot_matches_new_line, None);
    assert_eq!(result.swap_greed, None);
    assert_eq!(result.unicode, None);
}

