// Answer 0

#[test]
fn test_shortest_match_basic() {
    let regex = Regex::new(r"a+").unwrap();
    let text = b"aaaaa";
    let pos = regex.shortest_match(text);
    assert_eq!(pos, Some(1));
}

#[test]
fn test_shortest_match_no_match() {
    let regex = Regex::new(r"x+").unwrap();
    let text = b"aaaaa";
    let pos = regex.shortest_match(text);
    assert_eq!(pos, None);
}

#[test]
fn test_shortest_match_empty_string() {
    let regex = Regex::new(r"a+").unwrap();
    let text = b"";
    let pos = regex.shortest_match(text);
    assert_eq!(pos, None);
}

#[test]
fn test_shortest_match_at_start() {
    let regex = Regex::new(r"a+").unwrap();
    let text = b"aaaab";
    let pos = regex.shortest_match(text);
    assert_eq!(pos, Some(1));
}

#[test]
fn test_shortest_match_multiple_matches() {
    let regex = Regex::new(r"a+").unwrap();
    let text = b"abbcaaa";
    let pos = regex.shortest_match(text);
    assert_eq!(pos, Some(1));
}

#[test]
fn test_shortest_match_edge_case() {
    let regex = Regex::new(r"a+").unwrap();
    let text = b"a";
    let pos = regex.shortest_match(text);
    assert_eq!(pos, Some(1));
}

#[test]
fn test_shortest_match_non_byte_characters() {
    let regex = Regex::new(r"\D+").unwrap();
    let text = b"123abc";
    let pos = regex.shortest_match(text);
    assert_eq!(pos, Some(3));
}

