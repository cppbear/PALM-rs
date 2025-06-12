// Answer 0

#[test]
fn test_replacen_basic() {
    let regex = Regex::new("abc").unwrap();
    let text = "abcabcabc";
    let limit = 2;
    let rep = "X";
    regex.replacen(text, limit, rep);
}

#[test]
fn test_replacen_no_matches() {
    let regex = Regex::new("xyz").unwrap();
    let text = "abcabcabc";
    let limit = 2;
    let rep = "X";
    regex.replacen(text, limit, rep);
}

#[test]
fn test_replacen_limit_exceed() {
    let regex = Regex::new("abc").unwrap();
    let text = "abcabcabc";
    let limit = 5;
    let rep = "X";
    regex.replacen(text, limit, rep);
}

#[test]
fn test_replacen_edge_case_empty_text() {
    let regex = Regex::new("abc").unwrap();
    let text = "";
    let limit = 2;
    let rep = "X";
    regex.replacen(text, limit, rep);
}

#[test]
fn test_replacen_edge_case_non_overlapping() {
    let regex = Regex::new("def").unwrap();
    let text = "abcdefdef";
    let limit = 1;
    let rep = "X";
    regex.replacen(text, limit, rep);
}

