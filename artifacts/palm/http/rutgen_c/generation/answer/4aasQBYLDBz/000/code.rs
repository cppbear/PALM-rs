// Answer 0

#[test]
fn test_probe_distance_forward() {
    let mask: Size = 15; // Example mask
    let hash_value = HashValue(7); // HashValue within the mask range
    let current: usize = 10; // A current position that is forward of the desired position
    let result = probe_distance(mask, hash_value, current);
    assert_eq!(result, 3); // Expected steps forward from desired position
}

#[test]
fn test_probe_distance_at_desired_position() {
    let mask: Size = 15;
    let hash_value = HashValue(7); 
    let current: usize = desired_pos(mask, hash_value); // Current is at desired position
    let result = probe_distance(mask, hash_value, current);
    assert_eq!(result, 0); // No steps forward
}

#[test]
fn test_probe_distance_wraparound() {
    let mask: Size = 15; 
    let hash_value = HashValue(7);
    let current: usize = 0; // Current position wraps around
    let result = probe_distance(mask, hash_value, current);
    assert_eq!(result, (desired_pos(mask, hash_value) as usize) - current & mask as usize);
}

#[test]
fn test_probe_distance_edge_case() {
    let mask: Size = 15; 
    let hash_value = HashValue(0); // Minimal hash value
    let current: usize = 15; // Current at the edge
    let result = probe_distance(mask, hash_value, current);
    assert_eq!(result, 15); // Expected steps forward from desired position
}

