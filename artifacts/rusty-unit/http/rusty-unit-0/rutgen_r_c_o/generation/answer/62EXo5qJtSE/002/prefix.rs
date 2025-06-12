// Answer 0

#[test]
fn test_hash_elem_using_with_red_danger_and_minimum_key() {
    let hasher = RandomState::new();
    let danger = Danger::Red(hasher);
    let key: &u16 = &1;
    hash_elem_using(&danger, key);
}

#[test]
fn test_hash_elem_using_with_red_danger_and_maximum_key() {
    let hasher = RandomState::new();
    let danger = Danger::Red(hasher);
    let key: &u16 = &65535;
    hash_elem_using(&danger, key);
}

#[test]
fn test_hash_elem_using_with_red_danger_and_mid_range_key() {
    let hasher = RandomState::new();
    let danger = Danger::Red(hasher);
    let key: &u16 = &32768;
    hash_elem_using(&danger, key);
}

#[test]
fn test_hash_elem_using_with_red_danger_and_random_key() {
    let hasher = RandomState::new();
    let danger = Danger::Red(hasher);
    let key: &u16 = &12345;
    hash_elem_using(&danger, key);
}

#[test]
#[should_panic]
fn test_hash_elem_using_with_red_danger_and_zero_key() {
    let hasher = RandomState::new();
    let danger = Danger::Red(hasher);
    let key: &u16 = &0;
    hash_elem_using(&danger, key);
}

