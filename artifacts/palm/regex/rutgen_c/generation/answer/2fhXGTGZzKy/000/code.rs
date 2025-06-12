// Answer 0

#[test]
fn test_escape_into_no_meta_characters() {
    let input = "abcde";
    let mut output = String::new();
    escape_into(input, &mut output);
    assert_eq!(output, "abcde");
}

#[test]
fn test_escape_into_all_meta_characters() {
    let input = r"\.+*?()|[]{}^$#&-~";
    let mut output = String::new();
    escape_into(input, &mut output);
    assert_eq!(output, r"\\\.\+\*\?\(\)\|\[\]\{\}\^\$\#\&\-~");
}

#[test]
fn test_escape_into_mixed_characters() {
    let input = "abc*def+ghi?jkl(mno)pqr";
    let mut output = String::new();
    escape_into(input, &mut output);
    assert_eq!(output, "abc\\*def\\+ghi\\?jkl\\(mno\\)pqr");
}

#[test]
fn test_escape_into_empty_string() {
    let input = "";
    let mut output = String::new();
    escape_into(input, &mut output);
    assert_eq!(output, "");
}

#[test]
fn test_escape_into_with_special_characters() {
    let input = "error: a/c\\b + \"quote\" (test)";
    let mut output = String::new();
    escape_into(input, &mut output);
    assert_eq!(output, "error: a/c\\b \\+ \\\"quote\\\" \\(test\\)");
}

