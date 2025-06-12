// Answer 0

#[test]
fn test_is_meta_character() {
    assert_eq!(super::is_meta_character('\\'), true);
    assert_eq!(super::is_meta_character('*'), true);
    assert_eq!(super::is_meta_character(']'), true);
    assert_eq!(super::is_meta_character('['), true);
    assert_eq!(super::is_meta_character('('), true);
    assert_eq!(super::is_meta_character('#'), true);
    assert_eq!(super::is_meta_character('$'), true);
    assert_eq!(super::is_meta_character('}'), true);
    assert_eq!(super::is_meta_character('^'), true);
    assert_eq!(super::is_meta_character('|'), true);
    assert_eq!(super::is_meta_character('~'), true);
    assert_eq!(super::is_meta_character(')'), true);
    assert_eq!(super::is_meta_character('?'), true);
    assert_eq!(super::is_meta_character('+'), true);
    assert_eq!(super::is_meta_character('{'), true);
    assert_eq!(super::is_meta_character('.'), true);
    assert_eq!(super::is_meta_character('-'), true);
    assert_eq!(super::is_meta_character('&'), true);
}

