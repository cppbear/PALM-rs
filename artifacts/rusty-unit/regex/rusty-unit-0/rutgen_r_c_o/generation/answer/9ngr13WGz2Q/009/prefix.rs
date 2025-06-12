// Answer 0

#[test]
fn test_replacen_with_no_expansion_and_multiple_matches() {
    let regex = Regex::new("foo").unwrap();
    let text = b"foo bar foo baz";
    let limit = 0;
    let replacement = b"bar";
    let result = regex.replacen(text, limit, replacement);
}

#[test]
fn test_replacen_with_no_expansion_and_single_match() {
    let regex = Regex::new("cat").unwrap();
    let text = b"cat";
    let limit = 0;
    let replacement = b"dog";
    let result = regex.replacen(text, limit, replacement);
}

#[test]
fn test_replacen_with_no_expansion_and_exact_length() {
    let regex = Regex::new("abc").unwrap();
    let text = b"abcabcabc";
    let limit = 0;
    let replacement = b"xyz";
    let result = regex.replacen(text, limit, replacement);
}

#[test]
fn test_replacen_with_no_expansion_and_no_matches() {
    let regex = Regex::new("z").unwrap();
    let text = b"hello world";
    let limit = 0;
    let replacement = b"no_match";
    let result = regex.replacen(text, limit, replacement);
}

#[test]
fn test_replacen_with_no_expansion_and_full_text() {
    let regex = Regex::new("a").unwrap();
    let text = b"aaaaaa";
    let limit = 0;
    let replacement = b"x";
    let result = regex.replacen(text, limit, replacement);
}

