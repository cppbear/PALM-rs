// Answer 0

#[test]
fn test_captures_iter_valid_input() {
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = b"'Citizen Kane' (1941), 'The Wizard of Oz' (1939), 'M' (1931).";
    let captures = re.captures_iter(text);
}

#[test]
fn test_captures_iter_empty_input() {
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = b"";
    let captures = re.captures_iter(text);
}

#[test]
fn test_captures_iter_no_matches() {
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = b"'Inception' (2010) 'The Matrix' (1999) 'Nonexistent' ()";
    let captures = re.captures_iter(text);
}

#[test]
fn test_captures_iter_single_match() {
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = b"'The Godfather' (1972)";
    let captures = re.captures_iter(text);
}

#[test]
fn test_captures_iter_multiple_whitespace() {
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = b"'Pulp Fiction'   (1994),     'Fight Club' (1999)";
    let captures = re.captures_iter(text);
}

#[test]
fn test_captures_iter_special_characters() {
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = b"'The Lion King' (1994), 'Finding Nemo!' (2003)";
    let captures = re.captures_iter(text);
}

#[test]
fn test_captures_iter_year_out_of_range() {
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = b"'The Time Machine' (1899)";
    let captures = re.captures_iter(text);
}

#[test]
fn test_captures_iter_long_title() {
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = b"'A Very Long Movie Title That Exceeds Normal Length' (2022)";
    let captures = re.captures_iter(text);
}

#[test]
fn test_captures_iter_edge_case_year() {
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = b"'Early Days' (1900), 'New Age' (2023)";
    let captures = re.captures_iter(text);
}

