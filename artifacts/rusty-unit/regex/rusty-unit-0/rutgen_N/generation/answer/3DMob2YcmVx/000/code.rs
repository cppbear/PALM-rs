// Answer 0

#[test]
fn test_is_meta_character_backslash() {
    assert_eq!(regex_syntax::is_meta_character('\\'), true);
}

#[test]
fn test_is_meta_character_dot() {
    assert_eq!(regex_syntax::is_meta_character('.'), true);
}

#[test]
fn test_is_meta_character_plus() {
    assert_eq!(regex_syntax::is_meta_character('+'), true);
}

#[test]
fn test_is_meta_character_star() {
    assert_eq!(regex_syntax::is_meta_character('*'), true);
}

#[test]
fn test_is_meta_character_question() {
    assert_eq!(regex_syntax::is_meta_character('?'), true);
}

#[test]
fn test_is_meta_character_open_paren() {
    assert_eq!(regex_syntax::is_meta_character('('), true);
}

#[test]
fn test_is_meta_character_close_paren() {
    assert_eq!(regex_syntax::is_meta_character(')'), true);
}

#[test]
fn test_is_meta_character_pipe() {
    assert_eq!(regex_syntax::is_meta_character('|'), true);
}

#[test]
fn test_is_meta_character_open_bracket() {
    assert_eq!(regex_syntax::is_meta_character('['), true);
}

#[test]
fn test_is_meta_character_close_bracket() {
    assert_eq!(regex_syntax::is_meta_character(']'), true);
}

#[test]
fn test_is_meta_character_open_brace() {
    assert_eq!(regex_syntax::is_meta_character('{'), true);
}

#[test]
fn test_is_meta_character_close_brace() {
    assert_eq!(regex_syntax::is_meta_character('}'), true);
}

#[test]
fn test_is_meta_character_caret() {
    assert_eq!(regex_syntax::is_meta_character('^'), true);
}

#[test]
fn test_is_meta_character_dollar() {
    assert_eq!(regex_syntax::is_meta_character('$'), true);
}

#[test]
fn test_is_meta_character_hash() {
    assert_eq!(regex_syntax::is_meta_character('#'), true);
}

#[test]
fn test_is_meta_character_ampersand() {
    assert_eq!(regex_syntax::is_meta_character('&'), true);
}

#[test]
fn test_is_meta_character_dash() {
    assert_eq!(regex_syntax::is_meta_character('-'), true);
}

#[test]
fn test_is_meta_character_tilde() {
    assert_eq!(regex_syntax::is_meta_character('~'), true);
}

#[test]
fn test_is_meta_character_space() {
    assert_eq!(regex_syntax::is_meta_character(' '), false);
}

#[test]
fn test_is_meta_character_alphanumeric() {
    assert_eq!(regex_syntax::is_meta_character('a'), false);
    assert_eq!(regex_syntax::is_meta_character('1'), false);
}

#[test]
fn test_is_meta_character_empty_string() {
    assert_eq!(regex_syntax::is_meta_character('\0'), false);
}

