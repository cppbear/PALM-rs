// Answer 0

#[test]
fn test_div5_zero() {
    let result = div5(0);
    assert_eq!(result, 0);
}

#[test]
fn test_div5_positive() {
    let result = div5(10);
    assert_eq!(result, 2);
}

#[test]
fn test_div5_large_number() {
    let result = div5(1_000_000);
    assert_eq!(result, 200_000);
}

#[test]
fn test_div5_non_multiple_of_five() {
    let result = div5(9);
    assert_eq!(result, 1);
}

#[test]
fn test_div5_exact_multiple() {
    let result = div5(15);
    assert_eq!(result, 3);
}

