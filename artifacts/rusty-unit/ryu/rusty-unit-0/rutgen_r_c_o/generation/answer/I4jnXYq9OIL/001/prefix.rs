// Answer 0

#[test]
fn test_div5_zero() {
    let result = div5(0);
}

#[test]
fn test_div5_small() {
    let result = div5(1);
}

#[test]
fn test_div5_small_small() {
    let result = div5(2);
}

#[test]
fn test_div5_small_large() {
    let result = div5(4);
}

#[test]
fn test_div5_divisible() {
    let result = div5(5);
}

#[test]
fn test_div5_large() {
    let result = div5(10);
}

#[test]
fn test_div5_max() {
    let result = div5(u64::MAX);
}

