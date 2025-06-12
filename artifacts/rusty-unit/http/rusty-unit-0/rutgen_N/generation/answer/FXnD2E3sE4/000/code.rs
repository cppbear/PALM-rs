// Answer 0

#[derive(Debug)]
struct Size(u64);

#[derive(Debug)]
struct HashValue(u64);

fn desired_pos(mask: Size, hash: HashValue) -> usize {
    (hash.0 & mask.0) as usize
}

#[test]
fn test_desired_pos_with_zero_mask() {
    let mask = Size(0);
    let hash = HashValue(12345);
    assert_eq!(desired_pos(mask, hash), 0);
}

#[test]
fn test_desired_pos_with_non_zero_mask() {
    let mask = Size(15); // Binary: 1111
    let hash = HashValue(9); // Binary: 1001
    assert_eq!(desired_pos(mask, hash), 9 & 15); // Expect 9
}

#[test]
fn test_desired_pos_with_full_mask() {
    let mask = Size(u64::MAX); // All bits set
    let hash = HashValue(54321);
    assert_eq!(desired_pos(mask, hash), 54321); // Expect 54321
}

#[test]
fn test_desired_pos_with_large_values() {
    let mask = Size(0b1111_1111_1111_1111);
    let hash = HashValue(0b1010_1010_1010_1010);
    assert_eq!(desired_pos(mask, hash), 0b1010_1010_1010_1010 & 0b1111_1111_1111_1111); // Expect 0b1010_1010_1010_1010
}

