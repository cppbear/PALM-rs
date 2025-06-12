// Answer 0

#[test]
fn test_desired_pos_zero_mask_zero_hash() {
    let mask: Size = 0;
    let hash = HashValue(0);
    desired_pos(mask, hash);
}

#[test]
fn test_desired_pos_zero_mask_max_hash() {
    let mask: Size = 0;
    let hash = HashValue(32767);
    desired_pos(mask, hash);
}

#[test]
fn test_desired_pos_max_mask_zero_hash() {
    let mask: Size = 32767;
    let hash = HashValue(0);
    desired_pos(mask, hash);
}

#[test]
fn test_desired_pos_max_mask_max_hash() {
    let mask: Size = 32767;
    let hash = HashValue(32767);
    desired_pos(mask, hash);
}

#[test]
fn test_desired_pos_half_mask_half_hash() {
    let mask: Size = 16384; // half of max
    let hash = HashValue(16384); // half of max
    desired_pos(mask, hash);
}

#[test]
fn test_desired_pos_random_mask_random_hash() {
    let mask: Size = 12345;
    let hash = HashValue(6789);
    desired_pos(mask, hash);
}

#[test]
fn test_desired_pos_mask_less_than_hash() {
    let mask: Size = 6555;
    let hash = HashValue(12345);
    desired_pos(mask, hash);
}

#[test]
fn test_desired_pos_hash_less_than_mask() {
    let mask: Size = 32767;
    let hash = HashValue(100);
    desired_pos(mask, hash);
}

