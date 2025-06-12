// Answer 0

#[test]
fn test_new_with_distinct_bytes() {
    let pat = vec![1, 2, 3, 4, 5];
    let result = new(pat.clone());
    assert_eq!(result.pat, pat);
    assert_eq!(result.rare1, 1);
    assert_eq!(result.rare1i, 0);
    assert_eq!(result.rare2, 2);
    assert_eq!(result.rare2i, 1);
}

#[test]
fn test_new_with_duplicate_rare_bytes() {
    let pat = vec![1, 2, 2, 1, 3];
    let result = new(pat.clone());
    assert_eq!(result.pat, pat);
    assert_eq!(result.rare1, 1);
    assert_eq!(result.rare1i, 3);
    assert_eq!(result.rare2, 2);
    assert_eq!(result.rare2i, 1);
}

#[test]
fn test_new_with_all_identical_bytes() {
    let pat = vec![7, 7, 7, 7];
    let result = new(pat.clone());
    assert_eq!(result.pat, pat);
    assert_eq!(result.rare1, 7);
    assert_eq!(result.rare1i, 3);
    assert_eq!(result.rare2, 7);
    assert_eq!(result.rare2i, 3);
}

#[test]
fn test_new_with_single_byte() {
    let pat = vec![42];
    let result = new(pat.clone());
    assert_eq!(result.pat, pat);
    assert_eq!(result.rare1, 42);
    assert_eq!(result.rare1i, 0);
    assert_eq!(result.rare2, 42);
    assert_eq!(result.rare2i, 0);
}

#[test]
fn test_new_with_empty_pattern() {
    let pat: Vec<u8> = Vec::new();
    let result = new(pat);
    assert_eq!(result.pat.len(), 0);
}

