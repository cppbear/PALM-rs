// Answer 0

#[test]
fn test_regex_strings_empty() {
    struct RegexMock {
        res: Vec<String>,
    }

    let mock = RegexMock { res: Vec::new() };
    let result = mock.regex_strings();
    assert_eq!(result.len(), 0);
}

#[test]
fn test_regex_strings_single_element() {
    struct RegexMock {
        res: Vec<String>,
    }

    let mock = RegexMock { res: vec![String::from("a+b")] };
    let result = mock.regex_strings();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], "a+b");
}

#[test]
fn test_regex_strings_multiple_elements() {
    struct RegexMock {
        res: Vec<String>,
    }

    let mock = RegexMock { res: vec![String::from("a+b"), String::from("c*d"), String::from("e?f")] };
    let result = mock.regex_strings();
    assert_eq!(result.len(), 3);
    assert_eq!(result[0], "a+b");
    assert_eq!(result[1], "c*d");
    assert_eq!(result[2], "e?f");
}

