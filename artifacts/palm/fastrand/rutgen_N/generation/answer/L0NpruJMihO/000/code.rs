// Answer 0

#[test]
fn test_alphanumeric() {
    let result = fastrand::alphanumeric();
    assert!(result.is_ascii_alphanumeric(), "The result is not an alphanumeric character");
}

#[test]
fn test_multiple_alphanumeric() {
    let results: Vec<char> = (0..1000).map(|_| fastrand::alphanumeric()).collect();
    assert!(results.iter().all(|&c| c.is_ascii_alphanumeric()), "Not all results are alphanumeric characters");
}

