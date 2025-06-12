// Answer 0

#[test]
fn test_hash_elem_using_green() {
    let danger = Danger::Green;
    let key = "test_key";
    let result = hash_elem_using(&danger, &key);
    assert_eq!(result.0, (fnv::FnvHasher::default().finish() & ((MAX_SIZE as u64) - 1)) as u16);
}

#[test]
fn test_hash_elem_using_yellow() {
    let danger = Danger::Yellow;
    let key = "test_key";
    let result = hash_elem_using(&danger, &key);
    assert_eq!(result.0, (fnv::FnvHasher::default().finish() & ((MAX_SIZE as u64) - 1)) as u16);
}

#[test]
fn test_hash_elem_using_red() {
    let random_state = RandomState::new();
    let danger = Danger::Red(random_state);
    let key = "test_key";
    let result = hash_elem_using(&danger, &key);
    let mut hasher = random_state.build_hasher();
    key.hash(&mut hasher);
    let hash_value = hasher.finish() & ((MAX_SIZE as u64) - 1) as u16;
    assert_eq!(result.0, hash_value);
}

#[test]
fn test_hash_elem_using_empty_string() {
    let danger = Danger::Green;
    let key = "";
    let result = hash_elem_using(&danger, &key);
    assert_eq!(result.0, (fnv::FnvHasher::default().finish() & ((MAX_SIZE as u64) - 1)) as u16);
}

