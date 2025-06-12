// Answer 0

#[test]
fn test_replacen_with_expansion_no_match() {
    let regex = Regex::new("foo").unwrap();
    let text: &[u8] = b"bar baz";
    let limit = 1;
    let rep: &[u8] = b"qux";
    regex.replacen(text, limit, rep);
}

#[test]
fn test_replacen_with_expansion_partial_matches() {
    let regex = Regex::new("b[a-z]+").unwrap();
    let text: &[u8] = b"bar baz";
    let limit = 1;
    let rep: &[u8] = b"qux";
    regex.replacen(text, limit, rep);
}

#[test]
fn test_replacen_with_expansion_exceeding_limit() {
    let regex = Regex::new("b[a-z]+").unwrap();
    let text: &[u8] = b"bar baz bongo";
    let limit = 1;
    let rep: &[u8] = b"qux";
    regex.replacen(text, limit, rep);
}

#[test]
fn test_replacen_with_expansion_exact_limit() {
    let regex = Regex::new("b[a-z]+").unwrap();
    let text: &[u8] = b"bar baz bongo";
    let limit = 2;
    let rep: &[u8] = b"qux";
    regex.replacen(text, limit, rep);
}

#[test]
fn test_replacen_with_expansion_no_op() {
    let regex = Regex::new("xyz").unwrap();
    let text: &[u8] = b"abc def";
    let limit = 1;
    let rep: &[u8] = b"qux";
    regex.replacen(text, limit, rep);
}

