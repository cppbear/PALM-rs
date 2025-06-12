// Answer 0

#[test]
fn test_pos_new_zero_index() {
    let index = 0;
    let hash = HashValue(123);
    Pos::new(index, hash);
}

#[test]
fn test_pos_new_max_index() {
    let index = 32767;
    let hash = HashValue(456);
    Pos::new(index, hash);
}

#[test]
fn test_pos_new_middle_index() {
    let index = 16384;
    let hash = HashValue(789);
    Pos::new(index, hash);
}

#[test]
fn test_pos_new_large_hash_value() {
    let index = 100;
    let hash = HashValue(u16::MAX);
    Pos::new(index, hash);
}

#[test]
#[should_panic]
fn test_pos_new_exceeded_index() {
    let index = 32768; // This will cause a panic
    let hash = HashValue(1);
    Pos::new(index, hash);
}

