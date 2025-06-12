// Answer 0

#[test]
fn test_captures_iter_multiple_matches() {
    let re = regex::Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = "'Citizen Kane' (1941), 'The Wizard of Oz' (1939), 'M' (1931).";
    let captures: Vec<_> = re.captures_iter(text).collect();
    
    assert_eq!(captures.len(), 3);
    assert_eq!(&captures[0]["title"], "Citizen Kane");
    assert_eq!(&captures[0]["year"], "1941");
    assert_eq!(&captures[1]["title"], "The Wizard of Oz");
    assert_eq!(&captures[1]["year"], "1939");
    assert_eq!(&captures[2]["title"], "M");
    assert_eq!(&captures[2]["year"], "1931");
}

#[test]
fn test_captures_iter_no_matches() {
    let re = regex::Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = "No matches here!";
    let captures: Vec<_> = re.captures_iter(text).collect();
    
    assert_eq!(captures.len(), 0);
}

#[test]
fn test_captures_iter_edge_case_empty_string() {
    let re = regex::Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = "";
    let captures: Vec<_> = re.captures_iter(text).collect();
    
    assert_eq!(captures.len(), 0);
}

#[test]
fn test_captures_iter_boundary_case_no_year() {
    let re = regex::Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = "'Missing Year' () and 'No Year' (abcd).";
    let captures: Vec<_> = re.captures_iter(text).collect();
    
    assert_eq!(captures.len(), 1);
    assert_eq!(&captures[0]["title"], "Missing Year");
    assert_eq!(&captures[0]["year"], ""); // Expecting an empty string for year
}

