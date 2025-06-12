// Answer 0

#[test]
fn test_from_shared_invalid_uri_char_low() {
    let input = Bytes::from_static(b"invalid\x00uri");
    let _ = PathAndQuery::from_shared(input);
}

#[test]
fn test_from_shared_invalid_uri_char_space() {
    let input = Bytes::from_static(b"invalid uri");
    let _ = PathAndQuery::from_shared(input);
}

#[test]
fn test_from_shared_invalid_uri_char_less_than() {
    let input = Bytes::from_static(b"invalid<uri");
    let _ = PathAndQuery::from_shared(input);
}

#[test]
fn test_from_shared_invalid_uri_char_greater_than() {
    let input = Bytes::from_static(b"invalid>uri");
    let _ = PathAndQuery::from_shared(input);
}

#[test]
fn test_from_shared_invalid_uri_char_at_sign() {
    let input = Bytes::from_static(b"invalid@uri");
    let _ = PathAndQuery::from_shared(input);
}

#[test]
fn test_from_shared_invalid_uri_char_backtick() {
    let input = Bytes::from_static(b"invalid`uri");
    let _ = PathAndQuery::from_shared(input);
}

#[test]
fn test_from_shared_invalid_uri_char_open_brace() {
    let input = Bytes::from_static(b"invalid{uri");
    let _ = PathAndQuery::from_shared(input);
}

#[test]
fn test_from_shared_invalid_uri_char_close_brace() {
    let input = Bytes::from_static(b"invalid}uri");
    let _ = PathAndQuery::from_shared(input);
}

#[test]
fn test_from_shared_invalid_uri_char_exclamation() {
    let input = Bytes::from_static(b"invalid!uri");
    let _ = PathAndQuery::from_shared(input);
}

#[test]
fn test_from_shared_invalid_uri_char_query() {
    let input = Bytes::from_static(b"invalid?uri!");
    let _ = PathAndQuery::from_shared(input);
}

