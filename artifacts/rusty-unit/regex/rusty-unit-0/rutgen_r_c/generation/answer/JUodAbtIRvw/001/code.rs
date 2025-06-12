// Answer 0

#[test]
fn test_simple_fold_existing_case_mapping() {
    let c = 'A';
    let result = simple_fold(c);
    assert!(result.is_ok());
    let iter = result.unwrap();
    let folded: Vec<char> = iter.0.clone().collect();
    assert!(folded.contains(&'a')); // 'A' folds to 'a'
}

#[test]
fn test_simple_fold_no_case_mapping() {
    let c = 'Ϡ'; // Greek character with no simple folding
    let result = simple_fold(c);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.is_some());
}

#[test]
fn test_simple_fold_character_after_mapping() {
    let c = 'C'; // Character with a case mapping
    let result = simple_fold(c);
    assert!(result.is_ok());
    let iter = result.unwrap();
    let folded: Vec<char> = iter.0.clone().collect();
    assert!(folded.contains(&'c')); // 'C' folds to 'c'
}

#[test]
fn test_simple_fold_high_cp_no_mapping() {
    let c = '\u{FFFF}'; // High code point with no mapping
    let result = simple_fold(c);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.is_some());
}

#[test]
fn test_simple_fold_boundary_character() {
    let c = 'Z'; // Boundary character with a clear case mapping
    let result = simple_fold(c);
    assert!(result.is_ok());
    let iter = result.unwrap();
    let folded: Vec<char> = iter.0.clone().collect();
    assert!(folded.contains(&'z')); // 'Z' folds to 'z'
}

