// Answer 0

#[test]
fn test_shortest_match_single_match() {
    let text = b"aaaaa";
    let regex = regex::bytes::Regex::new(r"a+").unwrap();
    let pos = regex.shortest_match(text);
    assert_eq!(pos, Some(1));
}

#[test]
fn test_shortest_match_no_match() {
    let text = b"bbbbb";
    let regex = regex::bytes::Regex::new(r"a+").unwrap();
    let pos = regex.shortest_match(text);
    assert_eq!(pos, None);
}

#[test]
fn test_shortest_match_multiple_matches() {
    let text = b"abababab";
    let regex = regex::bytes::Regex::new(r"a+").unwrap();
    let pos = regex.shortest_match(text);
    assert_eq!(pos, Some(1));
}

#[test]
fn test_shortest_match_empty_text() {
    let text = b"";
    let regex = regex::bytes::Regex::new(r"a+").unwrap();
    let pos = regex.shortest_match(text);
    assert_eq!(pos, None);
}

#[test]
fn test_shortest_match_ending_match() {
    let text = b"baaaa";
    let regex = regex::bytes::Regex::new(r"a+").unwrap();
    let pos = regex.shortest_match(text);
    assert_eq!(pos, Some(1));
}

