// Answer 0

#[test]
fn test_parse_valid_regex() {
    let options = RegexOptions {
        pats: vec![String::from("a*b"), String::from("c+d")],
        size_limit: 1024,
        dfa_size_limit: 512,
        nest_limit: 10,
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        swap_greed: false,
        ignore_whitespace: false,
        unicode: false,
        octal: false,
    };
    let exec_builder = ExecBuilder::new_options(options);
    let result = exec_builder.parse();
    assert!(result.is_ok());
    let parsed = result.unwrap();
    assert_eq!(parsed.exprs.len(), 2);
}

#[test]
fn test_parse_empty_patterns() {
    let options = RegexOptions {
        pats: vec![],
        size_limit: 1024,
        dfa_size_limit: 512,
        nest_limit: 10,
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        swap_greed: false,
        ignore_whitespace: false,
        unicode: false,
        octal: false,
    };
    let exec_builder = ExecBuilder::new_options(options);
    let result = exec_builder.parse();
    assert!(result.is_ok());
    let parsed = result.unwrap();
    assert_eq!(parsed.exprs.len(), 0);
}

#[test]
fn test_parse_single_invalid_regex() {
    let options = RegexOptions {
        pats: vec![String::from("["), String::from("c+d")],
        size_limit: 1024,
        dfa_size_limit: 512,
        nest_limit: 10,
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        swap_greed: false,
        ignore_whitespace: false,
        unicode: false,
        octal: false,
    };
    let exec_builder = ExecBuilder::new_options(options);
    let result = exec_builder.parse();
    assert!(result.is_err());
    if let Err(Error::Syntax(err)) = result {
        assert!(err.contains("unexpected"));
    }
}

#[test]
fn test_parse_multiple_invalid_regexes() {
    let options = RegexOptions {
        pats: vec![String::from("["), String::from("c+d")],
        size_limit: 1024,
        dfa_size_limit: 512,
        nest_limit: 10,
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        swap_greed: false,
        ignore_whitespace: false,
        unicode: false,
        octal: false,
    };
    let exec_builder = ExecBuilder::new_options(options);
    let result = exec_builder.parse();
    assert!(result.is_err());
    if let Err(Error::Syntax(err)) = result {
        assert!(err.contains("unexpected"));
    }
}

#[test]
fn test_parse_partial_anchored_regex() {
    let options = RegexOptions {
        pats: vec![String::from("a^b"), String::from("c+d")],
        size_limit: 1024,
        dfa_size_limit: 512,
        nest_limit: 10,
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        swap_greed: false,
        ignore_whitespace: false,
        unicode: false,
        octal: false,
    };
    let exec_builder = ExecBuilder::new_options(options);
    let result = exec_builder.parse();
    assert!(result.is_ok());
    let parsed = result.unwrap();
    assert_eq!(parsed.prefixes, Literals::empty()); // Expect no prefixes due to anchored pattern
}

