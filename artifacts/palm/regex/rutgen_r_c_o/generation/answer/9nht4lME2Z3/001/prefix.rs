// Answer 0

#[test]
fn test_find_at_start_0() {
    let regex = Regex::new(r"abc").unwrap();
    let text: &[u8] = b"abcdef";
    regex.find_at(text, 0);
}

#[test]
fn test_find_at_start_1() {
    let regex = Regex::new(r"bc").unwrap();
    let text: &[u8] = b"abcdef";
    regex.find_at(text, 1);
}

#[test]
fn test_find_at_start_2() {
    let regex = Regex::new(r"cde").unwrap();
    let text: &[u8] = b"abcdef";
    regex.find_at(text, 2);
}

#[test]
fn test_find_at_start_5() {
    let regex = Regex::new(r"f").unwrap();
    let text: &[u8] = b"abcdef";
    regex.find_at(text, 5);
}

#[test]
fn test_find_at_start_equal_to_len() {
    let regex = Regex::new(r"f").unwrap();
    let text: &[u8] = b"abcdef";
    regex.find_at(text, text.len());
}

#[test]
fn test_find_at_empty_text() {
    let regex = Regex::new(r"abc").unwrap();
    let text: &[u8] = b"";
    regex.find_at(text, 0);
}

#[test]
fn test_find_at_single_character_text() {
    let regex = Regex::new(r"a").unwrap();
    let text: &[u8] = b"a";
    regex.find_at(text, 0);
}

#[test]
fn test_find_at_text_length_1024() {
    let regex = Regex::new(r"x").unwrap();
    let text: &[u8] = &[b'x'; 1024];
    regex.find_at(text, 0);
}

#[test]
fn test_find_at_non_matching_text() {
    let regex = Regex::new(r"xyz").unwrap();
    let text: &[u8] = b"abcdef";
    regex.find_at(text, 0);
}

