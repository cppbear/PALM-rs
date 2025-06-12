// Answer 0

#[test]
fn test_probe_distance_edge_case_zero_current() {
    let mask: Size = 0b1111; // Example mask
    let hash = HashValue(0); // Example hash value
    let current: usize = 0; // Current should be 0

    let result = probe_distance(mask, hash, current);
    assert_eq!(result, 0);
}

#[test]
fn test_probe_distance_non_zero_current() {
    let mask: Size = 0b1111; // Example mask
    let hash = HashValue(2); // Example hash value
    let current: usize = desired_pos(mask, hash) + 3; // 3 steps forward from desired position

    let result = probe_distance(mask, hash, current);
    assert_eq!(result, 3);
}

#[test]
fn test_probe_distance_wrap_around() {
    let mask: Size = 0b1111; // Example mask
    let hash = HashValue(0b0001); // Example hash value
    let current: usize = 0; // Current is at 0

    let result = probe_distance(mask, hash, current);
    assert_eq!(result, current.wrapping_sub(desired_pos(mask, hash)) & mask as usize);
}

#[test]
fn test_probe_distance_boundary_condition() {
    let mask: Size = 0b1111; // Example mask
    let hash = HashValue(15); // Maximum hash value
    let current: usize = desired_pos(mask, hash); // Current at desired position

    let result = probe_distance(mask, hash, current);
    assert_eq!(result, 0);
}

#[test]
fn test_probe_distance_large_value() {
    let mask: Size = 0b1111; // Example mask
    let hash = HashValue(8); // Example hash value
    let current: usize = 20; // Current is significantly larger

    let result = probe_distance(mask, hash, current);
    assert_eq!(result, (current.wrapping_sub(desired_pos(mask, hash)) & mask as usize));
}

