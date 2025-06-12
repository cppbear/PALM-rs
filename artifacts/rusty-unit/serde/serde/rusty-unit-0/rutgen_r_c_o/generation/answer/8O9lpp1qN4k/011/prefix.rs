// Answer 0

#[test]
fn test_unexpected_char_lowercase() {
    let content = Content::Char('a');
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_char_uppercase() {
    let content = Content::Char('Z');
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_char_digit() {
    let content = Content::Char('5');
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_char_special_exclamation() {
    let content = Content::Char('!');
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_char_special_at() {
    let content = Content::Char('@');
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_char_special_hash() {
    let content = Content::Char('#');
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_char_special_dollar() {
    let content = Content::Char('$');
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_char_special_percent() {
    let content = Content::Char('%');
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_char_special_caret() {
    let content = Content::Char('^');
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_char_special_and() {
    let content = Content::Char('&');
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_char_special_asterisk() {
    let content = Content::Char('*');
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_char_special_parenthesis_open() {
    let content = Content::Char('(');
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_char_special_parenthesis_close() {
    let content = Content::Char(')');
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_char_special_minus() {
    let content = Content::Char('-');
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_char_special_equals() {
    let content = Content::Char('=');
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_char_special_plus() {
    let content = Content::Char('+');
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_char_special_brace_open() {
    let content = Content::Char('{');
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_char_special_brace_close() {
    let content = Content::Char('}');
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_char_special_bracket_open() {
    let content = Content::Char('[');
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_char_special_bracket_close() {
    let content = Content::Char(']');
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_char_special_colon() {
    let content = Content::Char(':');
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_char_special_semicolon() {
    let content = Content::Char(';');
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_char_special_quote() {
    let content = Content::Char('"');
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_char_special_less_than() {
    let content = Content::Char('<');
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_char_special_greater_than() {
    let content = Content::Char('>');
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_char_special_comma() {
    let content = Content::Char(',');
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_char_special_period() {
    let content = Content::Char('.');
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_char_special_question() {
    let content = Content::Char('?');
    let _ = content.unexpected();
}

#[test]
fn test_unexpected_char_special_slash() {
    let content = Content::Char('/');
    let _ = content.unexpected();
}

