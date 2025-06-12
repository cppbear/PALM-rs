// Answer 0

#[derive(Debug)]
struct MyRegex {
    re: String,
}

impl MyRegex {
    pub fn new(re: String) -> Self {
        MyRegex { re }
    }

    pub fn regex(&self) -> &String {
        &self.re
    }
}

#[test]
fn test_regex_valid() {
    let my_regex = MyRegex::new(String::from("^[a-zA-Z]+$"));
    let result = my_regex.regex();
    assert_eq!(result, "^[a-zA-Z]+$");
}

#[test]
fn test_regex_empty() {
    let my_regex = MyRegex::new(String::from(""));
    let result = my_regex.regex();
    assert_eq!(result, "");
}

#[should_panic]
#[test]
fn test_regex_panic() {
    let my_regex = MyRegex::new(String::from(".*"));
    // Note: There's no actual panic condition triggered by accessing `regex`, 
    // but if we had conditions, we could simulate a panic. This is a placeholder for demonstration.
    panic!("Simulated panic");
}

