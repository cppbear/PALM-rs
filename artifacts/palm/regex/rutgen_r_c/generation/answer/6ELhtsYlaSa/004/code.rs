// Answer 0

#[test]
fn test_run_suffixes() -> Result<()> {
    #[derive(Deserialize)]
    struct TestArgs {
        cmd_ast: bool,
        cmd_hir: bool,
        cmd_prefixes: bool,
        cmd_suffixes: bool,
        cmd_anchors: bool,
        cmd_captures: bool,
        cmd_compile: bool,
        cmd_utf8_ranges: bool,
        arg_pattern: String,
        arg_patterns: Vec<String>,
        arg_class: String,
        flag_size_limit: usize,
        flag_bytes: bool,
        flag_dfa: bool,
        flag_dfa_reverse: bool,
        flag_all_literals: bool,
        flag_literal_limit: usize,
        flag_class_limit: usize,
        flag_lcp: bool,
        flag_lcs: bool,
        flag_searcher: bool,
    }

    let args = TestArgs {
        cmd_ast: false,
        cmd_hir: false,
        cmd_prefixes: false,
        cmd_suffixes: true,
        cmd_anchors: false,
        cmd_captures: false,
        cmd_compile: false,
        cmd_utf8_ranges: false,
        arg_pattern: String::from("pattern"),
        arg_patterns: vec![String::from("test_string_suffix")],
        arg_class: String::from("class_name"),
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

    let result = run(&args);
    assert!(result.is_ok());
    Ok(())
}

