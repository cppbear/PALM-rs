// Answer 0

#[test]
fn test_no_expansion_no_dollar() {
    let mut replacer: &str = "Hello, World!";
    let result = replacer.no_expansion();
    assert_eq!(result, Some(Cow::Borrowed("Hello, World!")));
}

#[test]
fn test_no_expansion_with_dollar() {
    let mut replacer: &str = "Hello, $name!";
    let result = replacer.no_expansion();
    assert_eq!(result, None);
}

