// Answer 0

#[test]
fn test_captures_iter() {
    use std::str;
    use regex::bytes::Regex;

    // Initialize a Regex instance for the movie title and release year pattern
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = b"'Citizen Kane' (1941), 'The Wizard of Oz' (1939), 'M' (1931).";

    // Collect the results from captures_iter
    let mut results = Vec::new();
    for caps in re.captures_iter(text) {
        let title = str::from_utf8(&caps["title"]).unwrap().to_string();
        let year = str::from_utf8(&caps["year"]).unwrap().to_string();
        results.push((title, year));
    }

    // Check the results
    assert_eq!(results.len(), 3);
    assert_eq!(results[0], ("Citizen Kane".to_string(), "1941".to_string()));
    assert_eq!(results[1], ("The Wizard of Oz".to_string(), "1939".to_string()));
    assert_eq!(results[2], ("M".to_string(), "1931".to_string()));
}

#[test]
fn test_captures_iter_empty_input() {
    use std::str;
    use regex::bytes::Regex;

    // Initialize a Regex instance for the movie title and release year pattern
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = b"";

    // Collect the results from captures_iter
    let results: Vec<_> = re.captures_iter(text).collect();

    // Check that there are no matches
    assert!(results.is_empty());
}

