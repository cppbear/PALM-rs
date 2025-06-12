// Answer 0

#[derive(Debug)]
struct Size(u64);

#[derive(Debug)]
struct HashValue(u64);

fn desired_pos(mask: Size, hash: HashValue) -> usize {
    (hash.0 & mask.0) as usize
}

#[test]
fn test_desired_pos_basic() {
    let mask = Size(0b1111);
    let hash = HashValue(0b1010);
    assert_eq!(desired_pos(mask, hash), 0b1010 & 0b1111);
}

#[test]
fn test_desired_pos_zero_mask() {
    let mask = Size(0);
    let hash = HashValue(0b1010);
    assert_eq!(desired_pos(mask, hash), 0b1010 & 0);
}

#[test]
fn test_desired_pos_zero_hash() {
    let mask = Size(0b1111);
    let hash = HashValue(0);
    assert_eq!(desired_pos(mask, hash), 0 & 0b1111);
}

#[test]
fn test_desired_pos_full_mask() {
    let mask = Size(u64::MAX);
    let hash = HashValue(0b11001);
    assert_eq!(desired_pos(mask, hash), 0b11001 & u64::MAX);
}

#[test]
fn test_desired_pos_boundary_values() {
    let mask = Size(1);
    let hash = HashValue(1);
    assert_eq!(desired_pos(mask, hash), 1 & 1);
    
    let mask = Size(1);
    let hash = HashValue(0);
    assert_eq!(desired_pos(mask, hash), 0 & 1);
}

