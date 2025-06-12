// Answer 0

#[test]
fn test_shortest_match_basic() {
    use regex::Regex;

    let regex = Regex::new(r"a+").unwrap();
    let text = "aaaaa";
    let pos = regex.shortest_match(text);
    assert_eq!(pos, Some(1));
}

#[test]
fn test_shortest_match_no_match() {
    use regex::Regex;

    let regex = Regex::new(r"b+").unwrap();
    let text = "aaaaa";
    let pos = regex.shortest_match(text);
    assert_eq!(pos, None);
}

#[test]
fn test_shortest_match_empty_string() {
    use regex::Regex;

    let regex = Regex::new(r"a+").unwrap();
    let text = "";
    let pos = regex.shortest_match(text);
    assert_eq!(pos, None);
}

#[test]
fn test_shortest_match_multiple_matches() {
    use regex::Regex;

    let regex = Regex::new(r"a+").unwrap();
    let text = "abaaa";
    let pos = regex.shortest_match(text);
    assert_eq!(pos, Some(1));
}

#[test]
fn test_shortest_match_boundary() {
    use regex::Regex;

    let regex = Regex::new(r"a+").unwrap();
    let text = "a";
    let pos = regex.shortest_match(text);
    assert_eq!(pos, Some(1));
}

