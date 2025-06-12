// Answer 0

#[test]
fn test_captures_valid_input_named_groups() {
    use regex::bytes::Regex;

    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = b"Not my favorite movie: 'Citizen Kane' (1941).";
    
    let caps = re.captures(text).unwrap();
    
    assert_eq!(&caps["title"], &b"Citizen Kane"[..]);
    assert_eq!(&caps["year"], &b"1941"[..]);
    assert_eq!(&caps[0], &b"'Citizen Kane' (1941)"[..]);
}

#[test]
fn test_captures_valid_input_indexing() {
    use regex::bytes::Regex;

    let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
    let text = b"Not my favorite movie: 'The Matrix' (1999).";

    let caps = re.captures(text).unwrap();
    
    assert_eq!(&caps[1], &b"The Matrix"[..]);
    assert_eq!(&caps[2], &b"1999"[..]);
    assert_eq!(&caps[0], &b"'The Matrix' (1999)"[..]);
}

#[test]
fn test_captures_no_match() {
    use regex::bytes::Regex;

    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = b"No movie here.";

    let caps = re.captures(text);
    
    assert!(caps.is_none());
}

#[test]
#[should_panic]
fn test_captures_invalid_index() {
    use regex::bytes::Regex;

    let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
    let text = b"Not my favorite movie: 'Interstellar' (2014).";
    
    let caps = re.captures(text).unwrap();
    
    // This will panic as index 3 does not exist
    let _ = &caps[3];
} 

#[test]
#[should_panic]
fn test_captures_invalid_named_group() {
    use regex::bytes::Regex;

    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = b"Not my favorite movie: 'Pulp Fiction' (1994).";
    
    let caps = re.captures(text).unwrap();
    
    // This will panic as the named group "director" does not exist
    let _ = &caps["director"];
} 

#[test]
fn test_captures_empty_string() {
    use regex::bytes::Regex;

    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = b"";

    let caps = re.captures(text);
    
    assert!(caps.is_none());
} 

#[test]
fn test_captures_multiple_matches() {
    use regex::bytes::Regex;

    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = b"'Movie One' (2001) and 'Movie Two' (2002).";

    let caps = re.captures(text).unwrap();

    assert_eq!(&caps["title"], &b"Movie One"[..]);
    assert_eq!(&caps["year"], &b"2001"[..]);
    assert_eq!(&caps[0], &b"'Movie One' (2001)"[..]);
} 

