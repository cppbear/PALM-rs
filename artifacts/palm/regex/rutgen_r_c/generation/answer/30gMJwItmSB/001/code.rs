// Answer 0

#[test]
fn test_exec_builder_new_single_regex() {
    let regex = "a*b";
    let exec_builder = ExecBuilder::new(regex);
    assert_eq!(exec_builder.options.pats.len(), 1);
    assert_eq!(exec_builder.options.pats[0], regex);
}

#[test]
fn test_exec_builder_new_many_regexes() {
    let regexes = ["a*b", "c*d"];
    let exec_builder = ExecBuilder::new_many(&regexes);
    assert_eq!(exec_builder.options.pats.len(), regexes.len());
    assert_eq!(exec_builder.options.pats[0], regexes[0]);
    assert_eq!(exec_builder.options.pats[1], regexes[1]);
}

#[test]
fn test_exec_builder_new_options() {
    let options = RegexOptions {
        pats: vec!["a*b".to_string()],
        size_limit: 1024,
        dfa_size_limit: 2048,
        nest_limit: 100,
        case_insensitive: true,
        multi_line: false,
        dot_matches_new_line: true,
        swap_greed: false,
        ignore_whitespace: false,
        unicode: true,
        octal: false,
    };
    let exec_builder = ExecBuilder::new_options(options.clone());
    assert_eq!(exec_builder.options, options);
}

#[test]
fn test_exec_builder_set_bounded_backtracking() {
    let regex = "a*b";
    let exec_builder = ExecBuilder::new(regex).bounded_backtracking();
    assert!(exec_builder.match_type.is_some());
}

#[test]
fn test_exec_builder_set_nfa() {
    let regex = "a*b";
    let exec_builder = ExecBuilder::new(regex).nfa();
    assert!(matches!(exec_builder.match_type, Some(MatchType::Nfa(_))));
}

#[test]
fn test_exec_builder_set_bytes() {
    let regex = "a*b";
    let exec_builder = ExecBuilder::new(regex).bytes(true);
    assert_eq!(exec_builder.bytes, true);
}

#[test]
fn test_exec_builder_set_only_utf8() {
    let regex = "a*b";
    let exec_builder = ExecBuilder::new(regex).only_utf8(false);
    assert_eq!(exec_builder.only_utf8, false);
}

#[test]
fn test_exec_builder_set_unicode() {
    let regex = "a*b";
    let exec_builder = ExecBuilder::new(regex).unicode(true);
    assert_eq!(exec_builder.options.unicode, true);
}

