// Answer 0

#[test]
fn test_hash_negint_min() {
    let value = N::NegInt(-9223372036854775808);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
}

#[test]
fn test_hash_negint_mid() {
    let value = N::NegInt(-123456789);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
}

#[test]
fn test_hash_negint_max() {
    let value = N::NegInt(-1);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
}

#[test]
fn test_hash_float_neg_zero() {
    let value = N::Float(-0.0);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
}

#[test]
fn test_hash_float_neg_small() {
    let value = N::Float(-0.1);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
}

#[test]
fn test_hash_float_neg_large() {
    let value = N::Float(-12345.678);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
}

