// Answer 0

#[test]
fn test_random_bool_zero() {
    let _ = random_bool(0.0);
}

#[test]
fn test_random_bool_half() {
    let _ = random_bool(0.5);
}

#[test]
fn test_random_bool_one() {
    let _ = random_bool(1.0);
}

#[test]
#[should_panic]
fn test_random_bool_negative() {
    let _ = random_bool(-0.1);
}

#[test]
#[should_panic]
fn test_random_bool_above_one() {
    let _ = random_bool(1.1);
}

