// Answer 0

#[test]
fn test_replace_simple_string() {
    let re = Regex::new("[^01]+").unwrap();
    let result = re.replace("1078910", "");
    assert_eq!(result, "1010");
}

#[test]
fn test_replace_with_captures() {
    struct ReplacerClosure;

    impl Replacer for ReplacerClosure {
        fn replace(&self, captures: &Captures, result: &mut String) {
            result.push_str(&captures[1]);
            result.push_str(" ");
            result.push_str(&captures[0]);
        }
    }

    let re = Regex::new(r"([^,\s]+),\s+(\S+)").unwrap();
    let result = re.replace("Springsteen, Bruce", ReplacerClosure);
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
fn test_replace_no_matched() {
    let re = Regex::new(r"no_match").unwrap();
    let result = re.replace("nothing to replace", "");
    assert_eq!(result, "nothing to replace");
}

