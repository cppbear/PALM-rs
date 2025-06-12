// Answer 0

#[test]
fn test_into_key_standard_case() {
    let mut map = HeaderMap::new();
    if let Entry::Vacant(v) = map.entry("x-hello") {
        let key = v.into_key();
    }
}

#[test]
fn test_into_key_empty_string() {
    let mut map = HeaderMap::new();
    if let Entry::Vacant(v) = map.entry("") {
        let key = v.into_key();
    }
}

#[test]
fn test_into_key_max_length_string() {
    let mut map = HeaderMap::new();
    let long_key = "x-".to_string() + &"a".repeat(61); // Total length 64
    if let Entry::Vacant(v) = map.entry(&long_key) {
        let key = v.into_key();
    }
}

#[test]
#[should_panic]
fn test_into_key_exceeding_max_length() {
    let mut map = HeaderMap::new();
    let long_key = "x-".to_string() + &"a".repeat(62); // Total length 65
    if let Entry::Vacant(v) = map.entry(&long_key) {
        let key = v.into_key();
    }
}

#[test]
fn test_into_key_special_character() {
    let mut map = HeaderMap::new();
    if let Entry::Vacant(v) = map.entry("x-hello@world.com") {
        let key = v.into_key();
    }
}

#[test]
fn test_into_key_numeric_string() {
    let mut map = HeaderMap::new();
    if let Entry::Vacant(v) = map.entry("123-abc") {
        let key = v.into_key();
    }
}

