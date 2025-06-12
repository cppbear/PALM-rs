// Answer 0

#[test]
fn test_replace_with_no_matches() {
    use regex::Regex;
    use std::borrow::Cow;

    let re = Regex::new(r"(\d+)").unwrap();
    let result = re.replace("hello", "number is $1");
    assert_eq!(result, Cow::Borrowed("hello"));
}

#[test]
fn test_replace_with_simple_match() {
    use regex::Regex;
    use std::borrow::Cow;

    let re = Regex::new(r"(\d+)").unwrap();
    let result = re.replace("I have 2 apples", "number is $1");
    assert_eq!(result, Cow::Owned("I have number is 2 apples".to_string()));
}

#[test]
fn test_replace_with_named_capture() {
    use regex::Regex;
    use std::borrow::Cow;

    let re = Regex::new(r"(?P<color>\w+) apples").unwrap();
    let result = re.replace("red apples", "$color are delicious");
    assert_eq!(result, Cow::Owned("red are delicious".to_string()));
}

#[test]
fn test_replace_with_multiple_capture_groups() {
    use regex::Regex;
    use std::borrow::Cow;

    let re = Regex::new(r"(?P<first>\w+) (?P<second>\w+)").unwrap();
    let result = re.replace("deep fried", "${first}_${second}");
    assert_eq!(result, Cow::Owned("deep_fried".to_string()));
}

#[test]
fn test_replace_with_function_replacer() {
    use regex::{Regex, Captures};
    use std::borrow::Cow;

    let re = Regex::new(r"(?P<first>\w+) (?P<second>\w+)").unwrap();
    let result = re.replace("hello world", |caps: &Captures| {
        format!("{} - {}", &caps["first"], &caps["second"])
    });
    assert_eq!(result, Cow::Owned("hello - world".to_string()));
}

#[test]
#[should_panic]
fn test_replace_with_invalid_capture() {
    use regex::Regex;
    use std::borrow::Cow;

    let re = Regex::new(r"(\d+)").unwrap();
    let _result = re.replace("I have 2 apples", "$10"); // Invalid capture group
}

#[test]
fn test_replace_with_literal_dollar() {
    use regex::Regex;
    use std::borrow::Cow;

    let re = Regex::new(r"(\d+)").unwrap();
    let result = re.replace("I have 2 apples", "$$1");
    assert_eq!(result, Cow::Owned("I have $2 apples".to_string()));
}

