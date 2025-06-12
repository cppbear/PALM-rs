// Answer 0

#[test]
fn test_find_iter_empty_text() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text: &[u8] = b"";
    let matches: Matches = regex.find_iter(text);
    assert_eq!(matches.0.count(), 0);
}

#[test]
fn test_find_iter_no_match() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text: &[u8] = b"short word";
    let matches: Matches = regex.find_iter(text);
    assert_eq!(matches.0.count(), 0);
}

#[test]
fn test_find_iter_one_match() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text: &[u8] = b"This is a verylongword indeed.";
    let matches: Matches = regex.find_iter(text);
    let match_positions: Vec<(usize, usize)> = matches.0.map(|m| (m.start(), m.end())).collect();
    assert_eq!(match_positions, vec![(10, 23)]);
}

#[test]
fn test_find_iter_multiple_matches() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text: &[u8] = b"word wordword longword abcdefghijklm";
    let matches: Matches = regex.find_iter(text);
    let match_positions: Vec<(usize, usize)> = matches.0.map(|m| (m.start(), m.end())).collect();
    assert_eq!(match_positions, vec![(0, 13), (14, 26), (27, 40)]);
}

#[test]
fn test_find_iter_boundary_match() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text: &[u8] = b"abcdefghijklm abcdefghijklmno";
    let matches: Matches = regex.find_iter(text);
    let match_positions: Vec<(usize, usize)> = matches.0.map(|m| (m.start(), m.end())).collect();
    assert_eq!(match_positions, vec![(0, 13)]);
}

