// Answer 0

#[derive(Debug)]
struct RegexWrapper(Vec<String>);

impl RegexWrapper {
    fn regex_strings(&self) -> &Vec<String> {
        &self.0
    }
}

impl RegexWrapper {
    pub fn as_str(&self) -> &str {
        &self.regex_strings()[0]
    }
}

#[test]
fn test_as_str_non_empty() {
    let regex = RegexWrapper(vec![String::from("abc"), String::from("123")]);
    assert_eq!(regex.as_str(), "abc");
}

#[test]
fn test_as_str_single_element() {
    let regex = RegexWrapper(vec![String::from("single")]);
    assert_eq!(regex.as_str(), "single");
}

#[test]
#[should_panic]
fn test_as_str_empty() {
    let regex = RegexWrapper(vec![]);
    let _ = regex.as_str();
}

