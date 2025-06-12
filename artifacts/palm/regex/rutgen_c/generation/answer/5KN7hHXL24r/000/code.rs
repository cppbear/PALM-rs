// Answer 0

#[test]
fn test_captures_iter() {
    use std::str;
    use regex::bytes::Regex;

    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = b"'Citizen Kane' (1941), 'The Wizard of Oz' (1939), 'M' (1931).";

    let mut captures_iter = re.captures_iter(text);
    
    let first_capture = captures_iter.next().unwrap();
    let title = str::from_utf8(&first_capture["title"]).unwrap();
    let year = str::from_utf8(&first_capture["year"]).unwrap();
    
    assert_eq!(title, "Citizen Kane");
    assert_eq!(year, "1941");
    
    let second_capture = captures_iter.next().unwrap();
    let title = str::from_utf8(&second_capture["title"]).unwrap();
    let year = str::from_utf8(&second_capture["year"]).unwrap();
    
    assert_eq!(title, "The Wizard of Oz");
    assert_eq!(year, "1939");
    
    let third_capture = captures_iter.next().unwrap();
    let title = str::from_utf8(&third_capture["title"]).unwrap();
    let year = str::from_utf8(&third_capture["year"]).unwrap();
    
    assert_eq!(title, "M");
    assert_eq!(year, "1931");

    assert!(captures_iter.next().is_none());
}

#[test]
#[should_panic]
fn test_captures_iter_no_match() {
    use regex::bytes::Regex;

    let re = Regex::new(r"'(?P<title>[^']+)'\s+\((?P<year>\d{4})\)").unwrap();
    let text = b"No matches here.";

    let mut captures_iter = re.captures_iter(text);

    // This should panic because we expect no captures to be found
    let _capture = captures_iter.next().unwrap();
}

