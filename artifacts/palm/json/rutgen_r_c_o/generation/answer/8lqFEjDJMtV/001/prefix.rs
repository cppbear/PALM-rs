// Answer 0

#[test]
fn test_hash_float_zero_positive() {
    let float_number = N::Float(0.0);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    float_number.hash(&mut hasher);
}

#[test]
fn test_hash_float_zero_negative() {
    let float_number = N::Float(-0.0);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    float_number.hash(&mut hasher);
}

#[test]
fn test_hash_float_negative_one() {
    let float_number = N::Float(-1.0);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    float_number.hash(&mut hasher);
}

#[test]
fn test_hash_float_positive_one() {
    let float_number = N::Float(1.0);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    float_number.hash(&mut hasher);
}

