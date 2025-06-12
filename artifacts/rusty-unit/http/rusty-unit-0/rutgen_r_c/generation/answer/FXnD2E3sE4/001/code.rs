// Answer 0

#[test]
fn test_desired_pos_normal_case() {
    let mask: Size = 0b1111; // 15 in decimal
    let hash = HashValue(0b1010); // 10 in decimal
    let result = desired_pos(mask, hash);
    assert_eq!(result, 10 & 15, "Expected result did not match the output.");
}

#[test]
fn test_desired_pos_zero_mask() {
    let mask: Size = 0; // 0 in decimal
    let hash = HashValue(0b1010); // 10 in decimal
    let result = desired_pos(mask, hash);
    assert_eq!(result, 0 & 0, "Expected result did not match the output.");
}

#[test]
fn test_desired_pos_mask_greater_than_hash() {
    let mask: Size = 0b1111; // 15 in decimal
    let hash = HashValue(0b0001); // 1 in decimal
    let result = desired_pos(mask, hash);
    assert_eq!(result, 1 & 15, "Expected result did not match the output.");
}

#[test]
fn test_desired_pos_full_mask() {
    let mask: Size = 0b1111; // 15 in decimal
    let hash = HashValue(0b1111); // 15 in decimal
    let result = desired_pos(mask, hash);
    assert_eq!(result, 15 & 15, "Expected result did not match the output.");
}

#[test]
fn test_desired_pos_exceeding_mask() {
    let mask: Size = 0b0111; // 7 in decimal
    let hash = HashValue(0b1111); // 15 in decimal
    let result = desired_pos(mask, hash);
    assert_eq!(result, 15 & 7, "Expected result did not match the output.");
}

