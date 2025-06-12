// Answer 0

#[test]
fn test_probe_distance_zero_values() {
    let mask: Size = 0;
    let hash = HashValue(0);
    let current: usize = 0;
    probe_distance(mask, hash, current);
}

#[test]
fn test_probe_distance_max_mask() {
    let mask: Size = 32767;
    let hash = HashValue(0);
    let current: usize = 0;
    probe_distance(mask, hash, current);
}

#[test]
fn test_probe_distance_max_hash() {
    let mask: Size = 32767;
    let hash = HashValue(32767);
    let current: usize = 0;
    probe_distance(mask, hash, current);
}

#[test]
fn test_probe_distance_varied_current() {
    let mask: Size = 16384;
    let hash = HashValue(8192);
    let current: usize = 4096;
    probe_distance(mask, hash, current);
}

#[test]
fn test_probe_distance_current_higher_than_mask() {
    let mask: Size = 16384;
    let hash = HashValue(8192);
    let current: usize = 32768;
    probe_distance(mask, hash, current);
}

#[test]
fn test_probe_distance_large_values() {
    let mask: Size = 32767;
    let hash = HashValue(32767);
    let current: usize = 32767;
    probe_distance(mask, hash, current);
}

#[test]
fn test_probe_distance_edge_cases() {
    let mask: Size = 1;
    let hash = HashValue(1);
    let current: usize = 2;
    probe_distance(mask, hash, current);
}

#[test]
fn test_probe_distance_reduced_mask() {
    let mask: Size = 32766;
    let hash = HashValue(1);
    let current: usize = 1;
    probe_distance(mask, hash, current);
}

