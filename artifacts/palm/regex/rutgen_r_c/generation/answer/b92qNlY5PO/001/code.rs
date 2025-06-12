// Answer 0

fn test_cmd_literals_invalid_input() -> Result<()> {
    struct FakeHir;
    
    impl Default for FakeHir {
        fn default() -> Self {
            FakeHir {}
        }
    }

    let invalid_args = Args {
        cmd_ast: false,
        cmd_hir: false,
        cmd_prefixes: false,
        cmd_suffixes: false,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: false,
        arg_pattern: String::from(""),
        arg_patterns: vec![],
        arg_class: String::from(""),
        flag_size_limit: 10485760,
        flag_bytes: false,
        flag_dfa: false,
        flag_dfa_reverse: false,
        flag_all_literals: false,
        flag_literal_limit: 250,
        flag_class_limit: 10,
        flag_lcp: false,
        flag_lcs: false,
        flag_searcher: false,
    };

    // Testing the case where parse_many would fail
    let result = cmd_literals(&invalid_args);
    assert!(result.is_err());

    Ok(())
}

fn test_cmd_literals_with_prefixes() -> Result<()> {
    struct FakeHir;

    impl Default for FakeHir {
        fn default() -> Self {
            FakeHir {}
        }
    }

    let valid_args = Args {
        cmd_ast: false,
        cmd_hir: false,
        cmd_prefixes: true,
        cmd_suffixes: false,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: false,
        arg_pattern: String::from("foo"),
        arg_patterns: vec![String::from("foo"), String::from("foobar")],
        arg_class: String::from("abc"),
        flag_size_limit: 10485760,
        flag_bytes: false,
        flag_dfa: false,
        flag_dfa_reverse: false,
        flag_all_literals: false,
        flag_literal_limit: 250,
        flag_class_limit: 10,
        flag_lcp: false,
        flag_lcs: false,
        flag_searcher: false,
    };

    let result = cmd_literals(&valid_args)?;
    assert!(result.is_ok());

    Ok(())
}

fn test_cmd_literals_with_suffixes() -> Result<()> {
    struct FakeHir;

    impl Default for FakeHir {
        fn default() -> Self {
            FakeHir {}
        }
    }

    let valid_args = Args {
        cmd_ast: false,
        cmd_hir: false,
        cmd_prefixes: false,
        cmd_suffixes: true,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: false,
        arg_pattern: String::from("bar"),
        arg_patterns: vec![String::from("bar"), String::from("foobar")],
        arg_class: String::from("abc"),
        flag_size_limit: 10485760,
        flag_bytes: false,
        flag_dfa: false,
        flag_dfa_reverse: false,
        flag_all_literals: false,
        flag_literal_limit: 250,
        flag_class_limit: 10,
        flag_lcp: false,
        flag_lcs: false,
        flag_searcher: false,
    };

    let result = cmd_literals(&valid_args)?;
    assert!(result.is_ok());

    Ok(())
}

