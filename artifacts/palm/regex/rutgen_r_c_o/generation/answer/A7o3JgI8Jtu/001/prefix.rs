// Answer 0

#[test]
fn test_case_fold_simple_empty_bytes() {
    let mut class = Class::Bytes(ClassBytes::empty());
    class.case_fold_simple();
}

#[test]
fn test_case_fold_simple_single_range() {
    let mut class = Class::Bytes(ClassBytes::new(vec![0..=127])); // ASCII range
    class.case_fold_simple();
}

#[test]
fn test_case_fold_simple_full_range() {
    let mut class = Class::Bytes(ClassBytes::new(vec![0..=255])); // All byte values
    class.case_fold_simple();
}

#[test]
fn test_case_fold_simple_mixed_ranges() {
    let mut class = Class::Bytes(ClassBytes::new(vec![65..=90, 97..=122])); // A-Z and a-z
    class.case_fold_simple();
}

#[test]
fn test_case_fold_simple_with_non_ascii() {
    let mut class = Class::Bytes(ClassBytes::new(vec![128..=255])); // Non-ASCII bytes
    class.case_fold_simple();
}

#[test]
#[should_panic]
fn test_case_fold_simple_invalid_range() {
    let mut class = Class::Bytes(ClassBytes::new(vec![200..=150])); // Invalid range
    class.case_fold_simple();
}

