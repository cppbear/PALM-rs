// Answer 0

#[derive(Debug)]
struct RegexWrapper {
    regex_strs: Vec<String>,
}

impl RegexWrapper {
    pub fn regex_strings(&self) -> &Vec<String> {
        &self.regex_strs
    }
}

struct MyRegex(RegexWrapper);

impl MyRegex {
    pub fn as_str(&self) -> &str {
        &self.0.regex_strings()[0]
    }
}

#[test]
fn test_as_str_single_string() {
    let regex = MyRegex(RegexWrapper {
        regex_strs: vec![String::from("single_string")]
    });
    assert_eq!(regex.as_str(), "single_string");
}

#[test]
fn test_as_str_multiple_strings() {
    let regex = MyRegex(RegexWrapper {
        regex_strs: vec![String::from("first_string"), String::from("second_string")]
    });
    assert_eq!(regex.as_str(), "first_string");
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_as_str_empty_strings() {
    let regex = MyRegex(RegexWrapper {
        regex_strs: vec![]
    });
    let _result = regex.as_str(); // This should panic
}

#[test]
fn test_as_str_with_special_characters() {
    let regex = MyRegex(RegexWrapper {
        regex_strs: vec![String::from("special*chars$@!")]
    });
    assert_eq!(regex.as_str(), "special*chars$@!");
}

