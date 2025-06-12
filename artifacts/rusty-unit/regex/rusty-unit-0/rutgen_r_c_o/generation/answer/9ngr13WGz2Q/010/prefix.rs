// Answer 0

#[test]
fn test_replacen_with_no_expansion_and_non_empty_matches() {
    let regex = Regex::new("1").unwrap();
    let text: &[u8] = &[1, 2, 3];
    let limit = 1;
    let rep: &[u8] = &[4];
    let _ = regex.replacen(text, limit, rep);
}

#[test]
fn test_replacen_with_no_expansion_and_multiple_matches() {
    let regex = Regex::new("2").unwrap();
    let text: &[u8] = &[1, 2, 2, 3];
    let limit = 1;
    let rep: &[u8] = &[4];
    let _ = regex.replacen(text, limit, rep);
}

#[test]
fn test_replacen_with_no_expansion_and_no_matches() {
    let regex = Regex::new("5").unwrap();
    let text: &[u8] = &[1, 2, 3];
    let limit = 1;
    let rep: &[u8] = &[4];
    let _ = regex.replacen(text, limit, rep);
}

#[test]
fn test_replacen_with_no_expansion_and_full_matches() {
    let regex = Regex::new("[1-3]").unwrap();
    let text: &[u8] = &[1, 2, 3];
    let limit = 3;
    let rep: &[u8] = &[4];
    let _ = regex.replacen(text, limit, rep);
}

