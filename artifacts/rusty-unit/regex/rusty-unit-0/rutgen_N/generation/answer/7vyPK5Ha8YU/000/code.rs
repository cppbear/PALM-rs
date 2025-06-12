// Answer 0

#[derive(Debug)]
struct RegexContainer<R> {
    re: R,
}

impl<R> RegexContainer<R> {
    fn new(re: R) -> Self {
        RegexContainer { re }
    }
}

#[test]
fn test_regex() {
    let regex_string = String::from(".*");
    let container = RegexContainer::new(regex_string);
    
    let result = container.regex();
    
    assert_eq!(result, &String::from(".*"));
}

#[test]
fn test_regex_with_different_type() {
    struct CustomRegex {
        pattern: String,
    }

    let custom_regex = CustomRegex { pattern: String::from("[a-z]+") };
    let container = RegexContainer::new(custom_regex);
    
    let result = container.regex();
    
    assert_eq!(result.pattern, "[a-z]+");
}

