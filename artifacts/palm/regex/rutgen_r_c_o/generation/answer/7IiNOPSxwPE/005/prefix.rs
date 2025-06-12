// Answer 0

#[test]
fn test_expand_str_with_named_capture() {
    let captures = re_unicode::Captures {
        text: "abcghi",
        locs: Locations::new(vec![(0, 3), (3, 6)]), // Example location of captures
        named_groups: Arc::new(HashMap::from([("name".to_string(), 0), ("other".to_string(), 1)])),
    };
    let mut dst = String::new();
    let replacement = "abc$name$def$other";
    expand_str(&captures, replacement, &mut dst);
}

#[test]
fn test_expand_str_with_multiple_named_captures() {
    let captures = re_unicode::Captures {
        text: "xyz",
        locs: Locations::new(vec![(0, 3)]), 
        named_groups: Arc::new(HashMap::from([("first".to_string(), 0), ("second".to_string(), 0)])),
    };
    let mut dst = String::new();
    let replacement = "$first$second";
    expand_str(&captures, replacement, &mut dst);
}

#[test]
fn test_expand_str_with_no_extra_dollars() {
    let captures = re_unicode::Captures {
        text: "12345",
        locs: Locations::new(vec![(0, 5)]),
        named_groups: Arc::new(HashMap::from([("num".to_string(), 0)])),
    };
    let mut dst = String::new();
    let replacement = "$num";
    expand_str(&captures, replacement, &mut dst);
}

#[test]
fn test_expand_str_with_consecutive_dollars() {
    let captures = re_unicode::Captures {
        text: "abcdefgh",
        locs: Locations::new(vec![(0, 8)]),
        named_groups: Arc::new(HashMap::from([("a".to_string(), 0)])),
    };
    let mut dst = String::new();
    let replacement = "$a$$$";
    expand_str(&captures, replacement, &mut dst);
}

#[test]
fn test_expand_str_with_empty_capture_name() {
    let captures = re_unicode::Captures {
        text: "test",
        locs: Locations::new(vec![(0, 4)]),
        named_groups: Arc::new(HashMap::new()),
    };
    let mut dst = String::new();
    let replacement = "$unknown$";
    expand_str(&captures, replacement, &mut dst);
}

