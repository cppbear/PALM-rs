// Answer 0

#[test]
fn test_run_cmd_compile() {
    struct TestArgs {
        cmd_ast: bool,
        cmd_hir: bool,
        cmd_prefixes: bool,
        cmd_suffixes: bool,
        cmd_anchors: bool,
        cmd_captures: bool,
        cmd_compile: bool,
        arg_pattern: String,
        arg_patterns: Vec<String>,
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

    impl TestArgs {
        fn new() -> Self {
            Self {
                cmd_ast: false,
                cmd_hir: false,
                cmd_prefixes: false,
                cmd_suffixes: false,
                cmd_anchors: false,
                cmd_captures: false,
                cmd_compile: true,
                arg_pattern: String::new(),
                arg_patterns: vec![String::from("abc"), String::from("123")],
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
            }
        }

        fn parse_many(&self) -> Result<Vec<String>> {
            Ok(self.arg_patterns.clone())
        }

        fn compiler(&self) -> Compiler {
            Compiler::new()
        }
    }

    let args = TestArgs::new();
    let result = run(&args);
    assert!(result.is_ok());
}

