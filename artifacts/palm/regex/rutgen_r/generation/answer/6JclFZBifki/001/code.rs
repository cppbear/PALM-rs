// Answer 0

#[test]
fn test_shortest_match_basic() {
    use regex::Regex;

    let text = "aaaaa";
    let regex = Regex::new(r"a+").unwrap();
    let pos = regex.shortest_match(text);
    assert_eq!(pos, Some(1));
}

#[test]
fn test_shortest_match_no_match() {
    use regex::Regex;

    let text = "bcccc";
    let regex = Regex::new(r"a+").unwrap();
    let pos = regex.shortest_match(text);
    assert_eq!(pos, None);
}

#[test]
fn test_shortest_match_single_char() {
    use regex::Regex;

    let text = "a";
    let regex = Regex::new(r"a+").unwrap();
    let pos = regex.shortest_match(text);
    assert_eq!(pos, Some(1));
}

#[test]
fn test_shortest_match_multiple_matches() {
    use regex::Regex;

    let text = "aaabaaa";
    let regex = Regex::new(r"a+").unwrap();
    let pos = regex.shortest_match(text);
    assert_eq!(pos, Some(1));
}

#[test]
fn test_shortest_match_empty_string() {
    use regex::Regex;

    let text = "";
    let regex = Regex::new(r"a+").unwrap();
    let pos = regex.shortest_match(text);
    assert_eq!(pos, None);
}

#[test]
fn test_shortest_match_non_matching_characters() {
    use regex::Regex;

    let text = "123456";
    let regex = Regex::new(r"a+").unwrap();
    let pos = regex.shortest_match(text);
    assert_eq!(pos, None);
}

#[test]
fn test_shortest_match_combined_characters() {
    use regex::Regex;

    let text = "abcdeaaaabc";
    let regex = Regex::new(r"a+").unwrap();
    let pos = regex.shortest_match(text);
    assert_eq!(pos, Some(4));
}

