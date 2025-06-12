// Answer 0

#[test]
fn test_unicode_enable() {
    let mut options = RegexOptions {
        pats: vec![],
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
    
    let exec_builder = ExecBuilder::new_options(options.clone());
    
    let exec_builder_unicode_enabled = exec_builder.unicode(true);
    assert!(exec_builder_unicode_enabled.options.unicode);
}

#[test]
fn test_unicode_disable() {
    let mut options = RegexOptions {
        pats: vec![],
        size_limit: 0,
        dfa_size_limit: 0,
        nest_limit: 0,
        case_insensitive: false,
        multi_line: false,
        dot_matches_new_line: false,
        swap_greed: false,
        ignore_whitespace: false,
        unicode: true,
        octal: false,
    };
    
    let exec_builder = ExecBuilder::new_options(options.clone());
    
    let exec_builder_unicode_disabled = exec_builder.unicode(false);
    assert!(!exec_builder_unicode_disabled.options.unicode);
}

