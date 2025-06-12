// Answer 0

#[test]
fn test_build_with_non_empty_patterns() {
    let options = RegexOptions {
        pats: vec![String::from("a*b"), String::from("c+d")],
        size_limit: 1024,
        dfa_size_limit: 512,
        nest_limit: 32,
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        swap_greed: false,
        ignore_whitespace: false,
        unicode: true,
        octal: false,
    };

    let exec_builder = ExecBuilder::new_options(options)
        .bytes(false)
        .only_utf8(true);

    let result = exec_builder.build();

    assert!(result.is_ok());

    let exec = result.unwrap();
    assert_eq!(exec.ro.match_type, MatchType::Nothing);
}

#[test]
fn test_build_with_valid_patterns_and_compile_success() {
    let options = RegexOptions {
        pats: vec![String::from("abc"), String::from("def")],
        size_limit: 2048,
        dfa_size_limit: 1024,
        nest_limit: 64,
        case_insensitive: true,
        multi_line: true,
        dot_matches_new_line: true,
        swap_greed: true,
        ignore_whitespace: true,
        unicode: false,
        octal: false,
    };

    let exec_builder = ExecBuilder::new_options(options)
        .nfa()
        .bounded_backtracking()
        .bytes(false)
        .only_utf8(false);

    let result = exec_builder.build();

    assert!(result.is_ok());

    let exec = result.unwrap();
    assert_eq!(exec.ro.match_type, MatchType::Nothing);
    assert!(!exec.ro.dfa.is_bytes);
}

#[test]
fn test_build_with_empty_suffixes() {
    let options = RegexOptions {
        pats: vec![String::from("x+y"), String::from("z*w")],
        size_limit: 4096,
        dfa_size_limit: 2048,
        nest_limit: 128,
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        swap_greed: false,
        ignore_whitespace: true,
        unicode: true,
        octal: false,
    };

    let exec_builder = ExecBuilder::new_options(options)
        .bytes(false)
        .only_utf8(true);

    let result = exec_builder.build();

    assert!(result.is_ok());

    let exec = result.unwrap();
    assert_eq!(exec.ro.suffixes.len(), 0);
}

#[test]
fn test_build_with_varied_patterns() {
    let options = RegexOptions {
        pats: vec![String::from("pattern1"), String::from("pattern2")],
        size_limit: 8192,
        dfa_size_limit: 4096,
        nest_limit: 256,
        case_insensitive: true,
        multi_line: true,
        dot_matches_new_line: true,
        swap_greed: false,
        ignore_whitespace: false,
        unicode: true,
        octal: false,
    };

    let exec_builder = ExecBuilder::new_options(options)
        .bytes(false)
        .only_utf8(false);

    let result = exec_builder.build();

    assert!(result.is_ok());

    let exec = result.unwrap();
    assert!(!exec.ro.nfa.is_bytes);
    assert!(exec.ro.dfa.is_dfa);
}

