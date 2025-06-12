// Answer 0

#[derive(Debug)]
struct RegexWrapper {
    regex_strings: Vec<String>,
}

impl RegexWrapper {
    fn new(regex_strings: Vec<String>) -> Self {
        Self { regex_strings }
    }

    fn regex_strings(&self) -> &Vec<String> {
        &self.regex_strings
    }
}

struct MyRegex(RegexWrapper);

impl MyRegex {
    pub fn as_str(&self) -> &str {
        &self.0.regex_strings()[0]
    }
}

#[test]
fn test_as_str_with_single_element() {
    let regex = MyRegex(RegexWrapper::new(vec![String::from("a*")]));
    assert_eq!(regex.as_str(), "a*");
}

#[test]
fn test_as_str_with_multiple_elements() {
    let regex = MyRegex(RegexWrapper::new(vec![String::from(".*"), String::from("^abc$")]));
    assert_eq!(regex.as_str(), ".*");
}

#[test]
#[should_panic]
fn test_as_str_on_empty_regex_strings() {
    let regex = MyRegex(RegexWrapper::new(vec![]));
    let _result = regex.as_str();
}

