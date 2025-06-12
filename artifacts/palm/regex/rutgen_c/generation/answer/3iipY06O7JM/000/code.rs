// Answer 0

#[test]
fn test_automatic_resets_match_type() {
    let regex_options = RegexOptions {
        pats: vec!["test".to_string()],
        size_limit: 0,
        dfa_size_limit: 0,
        nest_limit: 0,
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        swap_greed: false,
        ignore_whitespace: false,
        unicode: false,
        octal: false,
    };
    
    let exec_builder = ExecBuilder::new_options(regex_options.clone())
        .nfa() // Set match type to Nfa
        .bytes(true);

    let exec_builder_automatic = exec_builder.automatic();

    assert_eq!(exec_builder_automatic.match_type, None);
}

#[test]
fn test_automatic_when_no_type_set() {
    let regex_options = RegexOptions {
        pats: vec!["another_test".to_string()],
        size_limit: 0,
        dfa_size_limit: 0,
        nest_limit: 0,
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        swap_greed: false,
        ignore_whitespace: false,
        unicode: false,
        octal: false,
    };
    
    let exec_builder = ExecBuilder::new_options(regex_options);

    let exec_builder_automatic = exec_builder.automatic();

    assert_eq!(exec_builder_automatic.match_type, None);
}

