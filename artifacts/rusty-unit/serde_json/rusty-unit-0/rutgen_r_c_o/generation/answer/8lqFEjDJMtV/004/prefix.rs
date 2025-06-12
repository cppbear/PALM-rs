// Answer 0

#[test]
fn test_hash_posint_zero() {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    let number = N::PosInt(0);
    number.hash(&mut hasher);
}

#[test]
fn test_hash_posint_one() {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    let number = N::PosInt(1);
    number.hash(&mut hasher);
}

#[test]
fn test_hash_posint_max() {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    let number = N::PosInt(u64::MAX);
    number.hash(&mut hasher);
}

#[test]
fn test_hash_posint_large_value() {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    let number = N::PosInt(99999999999);
    number.hash(&mut hasher);
}

