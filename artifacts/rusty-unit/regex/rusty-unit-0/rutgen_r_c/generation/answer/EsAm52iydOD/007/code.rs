// Answer 0

#[test]
fn test_build_success_with_patterns() {
    let options = RegexOptions {
        pats: vec!["^abc".to_string(), "def$".to_string()],
        size_limit: 1024,
        dfa_size_limit: 512,
        nest_limit: 10,
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        swap_greed: false,
        ignore_whitespace: false,
        unicode: true,
        octal: false,
    };

    let exec_builder = ExecBuilder::new_options(options);

    match exec_builder.build() {
        Ok(exec) => {
            assert!(!exec.ro.nfa.insts.is_empty());
            assert!(!exec.ro.dfa.insts.is_empty());
            assert!(!exec.ro.dfa_reverse.insts.is_empty());
            assert_eq!(exec.ro.match_type, MatchType::Nothing);
        }
        Err(_) => panic!("Expected to build an Exec successfully but received an error."),
    }
}

#[test]
#[should_panic(expected = "CompiledTooBig")]
fn test_build_fails_on_compilation_too_big() {
    let options = RegexOptions {
        pats: vec![".*".to_string()], // Pattern that could lead to large compilation
        size_limit: 10, // Intentionally small size limit to trigger error
        dfa_size_limit: 512,
        nest_limit: 10,
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        swap_greed: false,
        ignore_whitespace: false,
        unicode: true,
        octal: false,
    };

    let exec_builder = ExecBuilder::new_options(options);
    
    let _ = exec_builder.build().unwrap(); // Expect this to panic
}

