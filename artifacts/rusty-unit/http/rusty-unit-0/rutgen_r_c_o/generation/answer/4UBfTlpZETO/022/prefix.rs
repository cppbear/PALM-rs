// Answer 0

#[test]
fn test_parse_empty_byte_slice() {
    let authority = Authority::empty();
    authority.parse(&[]); 
}

#[test]
fn test_parse_single_valid_character() {
    let authority = Authority::empty();
    authority.parse(&[b'a']); 
}

#[test]
fn test_parse_single_at_sign() {
    let authority = Authority::empty();
    authority.parse(&[b'@']); 
}

#[test]
fn test_parse_single_bracket_open() {
    let authority = Authority::empty();
    authority.parse(&[b'[']); 
}

#[test]
fn test_parse_single_bracket_close() {
    let authority = Authority::empty();
    authority.parse(&[b']']); 
}

#[test]
fn test_parse_single_percent() {
    let authority = Authority::empty();
    authority.parse(&[b'%']); 
}

#[test]
fn test_parse_single_valid_uri_char() {
    let authority = Authority::empty();
    authority.parse(&[b'c']); 
}

#[test]
fn test_parse_two_valid_uri_chars() {
    let authority = Authority::empty();
    authority.parse(&[b'a', b'b']); 
}

#[test]
fn test_parse_characters_up_to_at_sign() {
    let authority = Authority::empty();
    authority.parse(&[b'a', b'b', b'@']); 
}

#[test]
fn test_parse_colon_with_valid_characters() {
    let authority = Authority::empty();
    authority.parse(&[b'a', b':']); 
}

