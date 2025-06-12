// Answer 0

#[test]
fn test_ages_invalid_canonical_age() {
    let result = ages("V10_1");
}

#[test]
fn test_ages_invalid_canonical_age_empty() {
    let result = ages("");
}

#[test]
fn test_ages_invalid_canonical_age_large_value() {
    let result = ages("V11_0");
}

