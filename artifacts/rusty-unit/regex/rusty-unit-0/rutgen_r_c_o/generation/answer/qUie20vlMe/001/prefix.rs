// Answer 0

#[test]
fn test_unicode_setting_true() {
    let regex_options = RegexOptions {
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
    
    let exec_builder = ExecBuilder::new_options(regex_options);
    let _ = exec_builder.unicode(true);
}

#[test]
fn test_unicode_setting_false() {
    let regex_options = RegexOptions {
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
    
    let exec_builder = ExecBuilder::new_options(regex_options);
    let _ = exec_builder.unicode(false);
}

