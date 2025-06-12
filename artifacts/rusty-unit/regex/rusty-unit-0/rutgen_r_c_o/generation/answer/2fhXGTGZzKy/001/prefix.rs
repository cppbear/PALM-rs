// Answer 0

#[test]
fn test_escape_into_empty_string() {
    let mut buf = String::new();
    escape_into("", &mut buf);
}

#[test]
fn test_escape_into_meta_characters() {
    let meta_chars = vec!["\\", ".", "+", "*", "?", "(", ")", "|", "[", "]", "{", "}", "^", "$", "#", "&", "-", "~"];
    for &c in &meta_chars {
        let mut buf = String::new();
        escape_into(c, &mut buf);
    }
}

#[test]
fn test_escape_into_non_meta_characters() {
    let non_meta_chars = vec!["a", "b", "c", "1", "2", "3"];
    for &c in &non_meta_chars {
        let mut buf = String::new();
        escape_into(c, &mut buf);
    }
}

#[test]
fn test_escape_into_mixed_characters() {
    let mixed = "a.b+c?*()|[]{}^$#&-~";
    let mut buf = String::new();
    escape_into(mixed, &mut buf);
}

