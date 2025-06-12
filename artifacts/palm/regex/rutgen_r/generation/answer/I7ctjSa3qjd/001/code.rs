// Answer 0

#[test]
fn test_replace_append_valid_case() {
    use regex::Regex;
    use regex::Captures;

    let re = Regex::new(r"(?P<word>\w+)").unwrap();
    let caps = re.captures("Hello").unwrap();
    let mut output = String::new();

    re.replace_append(&caps, &mut output);
    
    assert_eq!(output, "Hello");
}

#[test]
#[should_panic]
fn test_replace_append_no_matches() {
    use regex::Regex;
    use regex::Captures;

    let re = Regex::new(r"(?P<word>\w+)").unwrap();
    let caps = Captures::empty();  // no captures

    let mut output = String::new();

    re.replace_append(&caps, &mut output);
}

#[test]
fn test_replace_append_edge_case() {
    use regex::Regex;
    use regex::Captures;

    let re = Regex::new(r"(?P<word>\w*)").unwrap();
    let caps = re.captures("").unwrap();
    let mut output = String::new();

    re.replace_append(&caps, &mut output);
    
    assert_eq!(output, "");
}

#[test]
fn test_replace_append_multiple_words() {
    use regex::Regex;
    use regex::Captures;

    let re = Regex::new(r"(?P<word>\w+)").unwrap();
    let inputs = vec!["Hello", "World"];
    let mut output = String::new();

    for &input in &inputs {
        let caps = re.captures(input).unwrap();
        re.replace_append(&caps, &mut output);
    }

    assert_eq!(output, "HelloWorld");
}

