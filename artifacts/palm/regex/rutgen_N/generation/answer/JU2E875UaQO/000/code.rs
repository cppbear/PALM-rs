// Answer 0

#[test]
fn test_replace_all_replaces_all_occurrences() {
    struct SimpleReplacer;

    impl Replacer for SimpleReplacer {
        fn replace(&self, _captures: &Captures) -> String {
            String::from("replacement")
        }
    }

    let regex = Regex::new(r"\d+").unwrap();
    let result = regex.replace_all("123 abc 456", SimpleReplacer);
    assert_eq!(result, "replacement abc replacement");
}

#[test]
fn test_replace_all_no_matches() {
    struct SimpleReplacer;

    impl Replacer for SimpleReplacer {
        fn replace(&self, _captures: &Captures) -> String {
            String::from("replacement")
        }
    }

    let regex = Regex::new(r"\d+").unwrap();
    let result = regex.replace_all("abc def", SimpleReplacer);
    assert_eq!(result, "abc def");
}

#[test]
fn test_replace_all_empty_string() {
    struct SimpleReplacer;

    impl Replacer for SimpleReplacer {
        fn replace(&self, _captures: &Captures) -> String {
            String::from("replacement")
        }
    }

    let regex = Regex::new(r"\d+").unwrap();
    let result = regex.replace_all("", SimpleReplacer);
    assert_eq!(result, "");
}

