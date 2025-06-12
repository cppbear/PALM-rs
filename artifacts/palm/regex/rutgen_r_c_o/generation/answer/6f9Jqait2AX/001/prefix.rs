// Answer 0

#[test]
fn test_captures_iter_basic() {
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = "'Some Title' (1990)";
    let _ = re.captures_iter(text);
}

#[test]
fn test_captures_iter_multiple() {
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = "'Movie One' (2001), 'Movie Two' (2005)";
    let _ = re.captures_iter(text);
}

#[test]
fn test_captures_iter_empty() {
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = "";
    let _ = re.captures_iter(text);
}

#[test]
fn test_captures_iter_no_matches() {
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = "No matches here!";
    let _ = re.captures_iter(text);
}

#[test]
fn test_captures_iter_invalid_format() {
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = "'Invalid Format' year 2022";
    let _ = re.captures_iter(text);
}

#[test]
fn test_captures_iter_missing_title() {
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = "' ' (1990)";
    let _ = re.captures_iter(text);
}

#[test]
fn test_captures_iter_edge_years() {
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = "'Edge Case' (1900), 'Future Case' (2023)";
    let _ = re.captures_iter(text);
}

#[test]
fn test_captures_iter_long_text() {
    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = "'Inception' (2010), 'Interstellar' (2014), 'Dunkirk' (2017), 'Tenet' (2020), 'Oppenheimer' (2023)";
    let _ = re.captures_iter(text);
}

