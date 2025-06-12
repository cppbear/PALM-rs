// Answer 0

#[test]
fn test_expand_str_basic_replacement() {
    let caps = re_unicode::Captures::new("test text", Locations::new(), Arc::new(HashMap::new()));
    let mut dst = String::new();
    expand_str(&caps, "Replace $1 and $name", &mut dst);
}

#[test]
fn test_expand_str_empty_replacement() {
    let caps = re_unicode::Captures::new("test text", Locations::new(), Arc::new(HashMap::new()));
    let mut dst = String::new();
    expand_str(&caps, "No replacements", &mut dst);
}

#[test]
fn test_expand_str_leading_dollar_sign() {
    let mut named_groups = Arc::new(HashMap::new());
    named_groups.insert("name".to_string(), 0);
    let caps = re_unicode::Captures::new("test text", Locations::new(), named_groups);
    let mut dst = String::new();
    expand_str(&caps, "$name is here", &mut dst);
}

#[test]
fn test_expand_str_consecutive_dollar_signs() {
    let mut named_groups = Arc::new(HashMap::new());
    named_groups.insert("name".to_string(), 0);
    let caps = re_unicode::Captures::new("test text", Locations::new(), named_groups);
    let mut dst = String::new();
    expand_str(&caps, "Value: $$value and $name", &mut dst);
}

#[test]
fn test_expand_str_no_dollar_followed() {
    let caps = re_unicode::Captures::new("test text", Locations::new(), Arc::new(HashMap::new()));
    let mut dst = String::new();
    expand_str(&caps, "This should not become $$", &mut dst);
}

#[test]
fn test_expand_str_empty_capture_ref() {
    let mut named_groups = Arc::new(HashMap::new());
    let caps = re_unicode::Captures::new("test text", Locations::new(), named_groups);
    let mut dst = String::new();
    expand_str(&caps, "Empty capture $missing", &mut dst);
}

#[test]
fn test_expand_str_complex_replacement() {
    let mut named_groups = Arc::new(HashMap::new());
    named_groups.insert("name".to_string(), 0);
    let caps = re_unicode::Captures::new("captured value", Locations::new(), named_groups);
    let mut dst = String::new();
    expand_str(&caps, "$0 is $name", &mut dst);
}

#[test]
fn test_expand_str_multiple_replacements() {
    let mut named_groups = Arc::new(HashMap::new());
    named_groups.insert("group1".to_string(), 1);
    named_groups.insert("group2".to_string(), 2);
    let caps = re_unicode::Captures::new("first second", Locations::new(), named_groups);
    let mut dst = String::new();
    expand_str(&caps, "$group1 and $group2", &mut dst);
}

