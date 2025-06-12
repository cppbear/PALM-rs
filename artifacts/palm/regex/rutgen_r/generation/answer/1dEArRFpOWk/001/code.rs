// Answer 0

#[test]
fn test_shortest_match_basic() {
    let regex = regex::bytes::Regex::new(r"a+").unwrap();
    let text = b"aaaaa";
    let pos = regex.shortest_match(text);
    assert_eq!(pos, Some(1));
}

#[test]
fn test_shortest_match_no_match() {
    let regex = regex::bytes::Regex::new(r"a+").unwrap();
    let text = b"bbbb";
    let pos = regex.shortest_match(text);
    assert_eq!(pos, None);
}

#[test]
fn test_shortest_match_empty_text() {
    let regex = regex::bytes::Regex::new(r"a+").unwrap();
    let text = b"";
    let pos = regex.shortest_match(text);
    assert_eq!(pos, None);
}

#[test]
fn test_shortest_match_multiple_matches() {
    let regex = regex::bytes::Regex::new(r"a+").unwrap();
    let text = b"aaabaaa";
    let pos = regex.shortest_match(text);
    assert_eq!(pos, Some(1));
}

#[test]
fn test_shortest_match_edge_case() {
    let regex = regex::bytes::Regex::new(r"a+").unwrap();
    let text = b"a";
    let pos = regex.shortest_match(text);
    assert_eq!(pos, Some(1));
}

