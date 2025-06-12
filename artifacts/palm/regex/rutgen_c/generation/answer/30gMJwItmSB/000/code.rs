// Answer 0

#[test]
fn test_exec_builder_new_single_regex() {
    let regex = "^abc$";
    let builder = ExecBuilder::new(regex);
    
    assert_eq!(builder.options.pats.len(), 1);
    assert_eq!(builder.options.pats[0], regex);
    assert!(builder.match_type.is_none());
    assert!(!builder.bytes);
    assert!(builder.only_utf8);
}

#[test]
fn test_exec_builder_new_many_regexes() {
    let regexes = vec!["^abc$", "def", "ghi"];
    let builder = ExecBuilder::new_many(regexes.iter());

    assert_eq!(builder.options.pats.len(), 3);
    assert_eq!(builder.options.pats[0], "^abc$");
    assert_eq!(builder.options.pats[1], "def");
    assert_eq!(builder.options.pats[2], "ghi");
    assert!(builder.match_type.is_none());
    assert!(!builder.bytes);
    assert!(builder.only_utf8);
}

#[test]
fn test_exec_builder_with_options() {
    let mut options = RegexOptions {
        pats: vec!["test".to_string()],
        size_limit: 1024,
        dfa_size_limit: 2048,
        nest_limit: 32,
        case_insensitive: true,
        multi_line: false,
        dot_matches_new_line: false,
        swap_greed: false,
        ignore_whitespace: true,
        unicode: false,
        octal: false,
    };

    let builder = ExecBuilder::new_options(options.clone());

    assert_eq!(builder.options, options);
    assert!(builder.match_type.is_none());
    assert!(!builder.bytes);
    assert!(builder.only_utf8);
}

