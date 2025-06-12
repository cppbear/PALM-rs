// Answer 0

#[test]
fn test_replace_with_string() {
    let re = Regex::new("[^01]+").unwrap();
    assert_eq!(re.replace("1078910", ""), "1010");
}

#[test]
fn test_replace_with_captures() {
    struct CapturesMock<'a>(&'a str);
    
    impl<'a> Replacer for fn(&Captures<'a>) -> String {
        fn replace(caps: &Captures<'a>) -> String {
            format!("{} {}", &caps[2], &caps[1])
        }
    }
    
    let re = Regex::new(r"([^,\s]+),\s+(\S+)").unwrap();
    let result = re.replace("Springsteen, Bruce", CapturesMock::replace);
    assert_eq!(result, "Bruce Springsteen");
}

#[test]
fn test_replace_with_named_captures() {
    let re = Regex::new(r"(?P<last>[^,\s]+),\s+(?P<first>\S+)").unwrap();
    let result = re.replace("Springsteen, Bruce", "$first $last");
    assert_eq!(result, "Bruce Springsteen");
}

#[test]
fn test_replace_with_curly_braces() {
    let re = Regex::new(r"(?P<first>\w+)\s+(?P<second>\w+)").unwrap();
    let result = re.replace("deep fried", "${first}_$second");
    assert_eq!(result, "deep_fried");
}

#[test]
fn test_replace_without_match() {
    let re = Regex::new(r"foo").unwrap();
    let result = re.replace("bar", "baz");
    assert_eq!(result, "bar");
}

#[test]
fn test_replace_with_invalid_capture() {
    let re = Regex::new(r"(?P<first>\w+)").unwrap();
    let result = re.replace("hello", "$2");
    assert_eq!(result, "hello");
}

