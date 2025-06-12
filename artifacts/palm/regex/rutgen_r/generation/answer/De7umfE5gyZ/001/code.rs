// Answer 0

#[test]
fn test_find_iter_with_exact_word_length() {
    use regex::bytes::Regex;

    let text = b"Retroactively relinquishing remunerations is reprehensible.";
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let matches: Vec<(usize, usize)> = regex.find_iter(text)
        .map(|m| (m.start(), m.end()))
        .collect();

    assert_eq!(matches, vec![(0, 13), (14, 27), (28, 41)]);
}

#[test]
fn test_find_iter_with_no_matches() {
    use regex::bytes::Regex;

    let text = b"Short words only.";
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let matches: Vec<(usize, usize)> = regex.find_iter(text)
        .map(|m| (m.start(), m.end()))
        .collect();

    assert_eq!(matches, vec![]);
}

#[test]
fn test_find_iter_with_empty_text() {
    use regex::bytes::Regex;

    let text: &[u8] = b"";
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let matches: Vec<(usize, usize)> = regex.find_iter(text)
        .map(|m| (m.start(), m.end()))
        .collect();

    assert_eq!(matches, vec![]);
}

#[test]
fn test_find_iter_with_multiple_matches() {
    use regex::bytes::Regex;

    let text = b"Longitudinal growths can happen.";
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let matches: Vec<(usize, usize)> = regex.find_iter(text)
        .map(|m| (m.start(), m.end()))
        .collect();

    assert_eq!(matches, vec![]);
}

#[test]
#[should_panic]
fn test_find_iter_with_invalid_regex() {
    use regex::bytes::Regex;

    let text = b"Testing failure.";
    let regex = Regex::new(r"[A-Z{13}]").unwrap(); // intentionally incorrect pattern
    let _ = regex.find_iter(text);
}

