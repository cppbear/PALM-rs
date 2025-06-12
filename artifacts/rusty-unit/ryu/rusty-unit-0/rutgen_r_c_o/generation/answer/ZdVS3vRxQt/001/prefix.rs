// Answer 0

#[test]
fn test_div10_zero() {
    let input = 0u64;
    div10(input);
}

#[test]
fn test_div10_small_value() {
    let input = 5u64; 
    div10(input);
}

#[test]
fn test_div10_ten() {
    let input = 10u64;
    div10(input);
}

#[test]
fn test_div10_large_value() {
    let input = 1000u64;
    div10(input);
}

#[test]
fn test_div10_max_value() {
    let input = u64::MAX;
    div10(input);
}

#[test]
fn test_div10_exact_division() {
    let input = 50u64; 
    div10(input);
}

#[test]
fn test_div10_just_above_ten() {
    let input = 11u64; 
    div10(input);
}

#[test]
fn test_div10_non_multiple_of_ten() {
    let input = 29u64; 
    div10(input);
}

