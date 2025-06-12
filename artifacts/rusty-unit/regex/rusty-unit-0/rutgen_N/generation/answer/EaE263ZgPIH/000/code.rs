// Answer 0

#[test]
fn test_escape_empty_string() {
    let result = regex_syntax::escape("");
    assert_eq!(result, "");
}

#[test]
fn test_escape_no_meta_characters() {
    let result = regex_syntax::escape("normal text");
    assert_eq!(result, "normal text");
}

#[test]
fn test_escape_all_meta_characters() {
    let result = regex_syntax::escape(".^$*+?{}()|[]\\");
    assert_eq!(result, "\\.\\^\\$\\*\\+\\?\\{\\}\\(\\)\\|\\[\\]\\\\");
}

#[test]
fn test_escape_combined() {
    let result = regex_syntax::escape("text with meta: . * + ? ( ) [ ] { } ^ $ | \\");
    assert_eq!(result, "text with meta: \\., \\* , \\+ , \\? , \\( , \\) , \\[ , \\] , \\{ , \\} , \\^ , \\$ , \\| , \\\\");
}

