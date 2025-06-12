// Answer 0

#[test]
fn test_simple_fold_existing_codepoint() {
    let result = simple_fold('a');
    assert!(result.is_ok());
    let iter = result.unwrap();
    let folds: Vec<_> = iter.collect();
    assert_eq!(folds, ['A']);
}

#[test]
fn test_simple_fold_non_existing_codepoint() {
    let result = simple_fold('z');
    assert!(result.is_err());
    assert_eq!(result.err(), Some('y'));
}

#[test]
fn test_simple_fold_empty_equivalence_class() {
    let result = simple_fold('\u{FFFF}');
    assert!(result.is_err());
    assert_eq!(result.err(), None);
}

#[test]
fn test_simple_fold_next_valid_codepoint() {
    let result = simple_fold('©');
    assert!(result.is_err());
    assert_eq!(result.err(), Some('S'));
}

