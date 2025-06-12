// Answer 0

#[test]
fn test_captures_iter_multiple_matches() {
    let re = regex::bytes::Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = b"'Citizen Kane' (1941), 'The Wizard of Oz' (1939), 'M' (1931).";
    
    let captures: Vec<_> = re.captures_iter(text).collect();
    assert_eq!(captures.len(), 3);

    assert_eq!(&captures[0]["title"], b"Citizen Kane");
    assert_eq!(&captures[0]["year"], b"1941");
    
    assert_eq!(&captures[1]["title"], b"The Wizard of Oz");
    assert_eq!(&captures[1]["year"], b"1939");
    
    assert_eq!(&captures[2]["title"], b"M");
    assert_eq!(&captures[2]["year"], b"1931");
}

#[test]
fn test_captures_iter_no_matches() {
    let re = regex::bytes::Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = b"No matches here.";
    
    let captures: Vec<_> = re.captures_iter(text).collect();
    assert_eq!(captures.len(), 0);
}

#[test]
fn test_captures_iter_empty_input() {
    let re = regex::bytes::Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text: &[u8] = b"";
    
    let captures: Vec<_> = re.captures_iter(text).collect();
    assert_eq!(captures.len(), 0);
}

#[test]
#[should_panic]
fn test_captures_iter_invalid_utf8() {
    let re = regex::bytes::Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = b"'\xF0\x9F\x92\xa9' (2021)"; // Invalid UTF-8 sequence
    
    let _ = re.captures_iter(text).collect::<Vec<_>>(); // Should panic on invalid UTF-8 conversion
}

