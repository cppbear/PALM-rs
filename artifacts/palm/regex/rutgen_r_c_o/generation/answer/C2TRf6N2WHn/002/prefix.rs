// Answer 0

#[test]
fn test_canonical_binary_valid_script_name() {
    let query = ClassQuery::Binary("valid_script_name");
    let result = query.canonical_binary("Latin");
}

#[test]
fn test_canonical_binary_another_valid_script_name() {
    let query = ClassQuery::Binary("another_valid_script_name");
    let result = query.canonical_binary("Greek");
}

#[test]
fn test_canonical_binary_short_valid_script_name() {
    let query = ClassQuery::Binary("S");
    let result = query.canonical_binary("Han");
}

#[test]
fn test_canonical_binary_long_valid_script_name() {
    let query = ClassQuery::Binary("long_valid_script_name_exceeding_length");
    let result = query.canonical_binary("Cyrillic");
}

#[test]
fn test_canonical_binary_unicode_characters_script() {
    let query = ClassQuery::Binary("حروف_الكثير");
    let result = query.canonical_binary("Arabic");
}

