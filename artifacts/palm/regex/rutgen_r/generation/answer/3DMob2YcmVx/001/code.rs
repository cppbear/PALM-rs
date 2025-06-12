// Answer 0

#[test]
fn test_is_meta_character() {
    assert_eq!(is_meta_character('*'), true);
    assert_eq!(is_meta_character(']'), true);
    assert_eq!(is_meta_character('['), true);
    assert_eq!(is_meta_character('('), true);
    assert_eq!(is_meta_character('#'), true);
    assert_eq!(is_meta_character('$'), true);
    assert_eq!(is_meta_character('\\'), true);
    assert_eq!(is_meta_character('}'), true);
    assert_eq!(is_meta_character('^'), true);
    assert_eq!(is_meta_character('|'), true);
    assert_eq!(is_meta_character('~'), true);
    assert_eq!(is_meta_character(')'), true);
    assert_eq!(is_meta_character('?'), true);
    assert_eq!(is_meta_character('+'), true);
    assert_eq!(is_meta_character('{'), true);
    assert_eq!(is_meta_character('.'), true);
    assert_eq!(is_meta_character('-'), true);
    assert_eq!(is_meta_character('&'), true);
    assert_eq!(is_meta_character('a'), false);
    assert_eq!(is_meta_character(' '), false);
}

