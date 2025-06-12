// Answer 0

#[test]
fn test_nfa_sets_match_type_to_pikevm() {
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

    let exec_builder = ExecBuilder::new_options(regex_options);
    let updated_builder = exec_builder.nfa();

    assert_eq!(updated_builder.match_type, Some(MatchType::Nfa(MatchNfaType::PikeVM)));
}

#[test]
fn test_nfa_overrides_previous_match_type() {
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

    let exec_builder = ExecBuilder::new_options(regex_options).automatic();
    let updated_builder = exec_builder.nfa();

    assert_eq!(updated_builder.match_type, Some(MatchType::Nfa(MatchNfaType::PikeVM)));
}

