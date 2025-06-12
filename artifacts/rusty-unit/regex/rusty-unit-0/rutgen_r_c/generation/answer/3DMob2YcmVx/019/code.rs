// Answer 0

#[test]
fn test_is_meta_character_backslash() {
    assert_eq!(is_meta_character('\\'), true);
}

#[test]
fn test_is_meta_character_asterisk() {
    assert_eq!(is_meta_character('*'), true);
}

#[test]
fn test_is_meta_character_close_square_bracket() {
    assert_eq!(is_meta_character(']'), true);
}

#[test]
fn test_is_meta_character_open_square_bracket() {
    assert_eq!(is_meta_character('['), true);
}

#[test]
fn test_is_meta_character_open_parenthesis() {
    assert_eq!(is_meta_character('('), true);
}

#[test]
fn test_is_meta_character_hash() {
    assert_eq!(is_meta_character('#'), true);
}

#[test]
fn test_is_meta_character_dollar_sign() {
    assert_eq!(is_meta_character('$'), true);
}

#[test]
fn test_is_meta_character_close_brace() {
    assert_eq!(is_meta_character('}'), true);
}

#[test]
fn test_is_meta_character_caret() {
    assert_eq!(is_meta_character('^'), true);
}

#[test]
fn test_is_meta_character_pipe() {
    assert_eq!(is_meta_character('|'), true);
}

#[test]
fn test_is_meta_character_tilde() {
    assert_eq!(is_meta_character('~'), true);
}

#[test]
fn test_is_meta_character_close_parenthesis() {
    assert_eq!(is_meta_character(')'), true);
}

#[test]
fn test_is_meta_character_question_mark() {
    assert_eq!(is_meta_character('?'), true);
}

#[test]
fn test_is_meta_character_plus() {
    assert_eq!(is_meta_character('+'), true);
}

#[test]
fn test_is_meta_character_open_brace() {
    assert_eq!(is_meta_character('{'), true);
}

#[test]
fn test_is_meta_character_dot() {
    assert_eq!(is_meta_character('.'), true);
}

#[test]
fn test_is_meta_character_dash() {
    assert_eq!(is_meta_character('-'), true);
}

#[test]
fn test_is_meta_character_ampersand() {
    assert_eq!(is_meta_character('&'), true);
}

