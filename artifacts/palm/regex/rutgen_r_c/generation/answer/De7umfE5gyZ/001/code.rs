// Answer 0

#[test]
fn test_find_iter_empty_text() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text: &[u8] = b"";
    let matches = regex.find_iter(text);
    assert!(matches.0.is_empty());
}

#[test]
fn test_find_iter_no_matches() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text: &[u8] = b"short words";
    let matches = regex.find_iter(text);
    assert!(matches.0.is_empty());
}

#[test]
fn test_find_iter_exact_match() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text: &[u8] = b"Retroactively";
    let matches = regex.find_iter(text);
    assert_eq!(matches.0.collect::<Vec<_>>(), vec![(0, 13)]);
}

#[test]
fn test_find_iter_multiple_matches() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text: &[u8] = b"Retroactively relinquishing remunerations";
    let matches = regex.find_iter(text);
    assert_eq!(matches.0.collect::<Vec<_>>(), vec![(0, 13), (14, 31)]);
}

#[test]
fn test_find_iter_overlapping_words() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text: &[u8] = b"overlapping overlappingwords";
    let matches = regex.find_iter(text);
    assert!(matches.0.is_empty());
}

#[test]
fn test_find_iter_unicode_characters() {
    let regex = Regex::new(r"\b\w{13}\b").unwrap();
    let text: &[u8] = "Retroactively挑戰挑戰挑戰".as_bytes();
    let matches = regex.find_iter(text);
    assert_eq!(matches.0.collect::<Vec<_>>(), vec![(0, 13)]);
}

