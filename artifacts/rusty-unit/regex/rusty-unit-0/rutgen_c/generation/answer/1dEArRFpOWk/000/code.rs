// Answer 0

#[test]
fn test_shortest_match_found() {
    let regex = Regex::new(r"a+").unwrap();
    let text = b"aaaaa";
    let pos = regex.shortest_match(text);
    assert_eq!(pos, Some(1));
}

#[test]
fn test_shortest_match_not_found() {
    let regex = Regex::new(r"x+").unwrap();
    let text = b"aaaaa";
    let pos = regex.shortest_match(text);
    assert_eq!(pos, None);
}

#[test]
fn test_shortest_match_empty_text() {
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
fn test_shortest_match_with_non_matching_start() {
    let regex = Regex::new(r"a+").unwrap();
    let text = b"bbaaa";
    let pos = regex.shortest_match(text);
    assert_eq!(pos, Some(3));
}

