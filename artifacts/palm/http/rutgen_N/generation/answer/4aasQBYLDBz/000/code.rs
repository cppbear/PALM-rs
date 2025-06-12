// Answer 0

#[test]
fn test_probe_distance_zero_steps() {
    struct Size(u64);
    struct HashValue(u64);

    fn desired_pos(mask: Size, hash: HashValue) -> usize {
        (hash.0 as usize) & (mask.0 as usize)
    }

    let mask = Size(0b111);
    let hash = HashValue(0b101);
    let current = desired_pos(mask, hash);

    assert_eq!(probe_distance(mask, hash, current), 0);
}

#[test]
fn test_probe_distance_boundary_steps() {
    struct Size(u64);
    struct HashValue(u64);

    fn desired_pos(mask: Size, hash: HashValue) -> usize {
        (hash.0 as usize) & (mask.0 as usize)
    }

    let mask = Size(0b111);
    let hash = HashValue(0b110);
    let current = desired_pos(mask, hash) + 7; // Test wrapping behavior

    assert_eq!(probe_distance(mask, hash, current), 1);
}

#[test]
fn test_probe_distance_multiple_steps() {
    struct Size(u64);
    struct HashValue(u64);

    fn desired_pos(mask: Size, hash: HashValue) -> usize {
        (hash.0 as usize) & (mask.0 as usize)
    }

    let mask = Size(0b111);
    let hash = HashValue(0b111);
    let current = desired_pos(mask, hash) + 4; // Test multiple steps ahead

    assert_eq!(probe_distance(mask, hash, current), 3);
}

#[test]
fn test_probe_distance_wrap_around() {
    struct Size(u64);
    struct HashValue(u64);

    fn desired_pos(mask: Size, hash: HashValue) -> usize {
        (hash.0 as usize) & (mask.0 as usize)
    }

    let mask = Size(0b111);
    let hash = HashValue(0b010);
    let current = desired_pos(mask, hash) + 8; // Test wrapping around back to start

    assert_eq!(probe_distance(mask, hash, current), 2);
}

