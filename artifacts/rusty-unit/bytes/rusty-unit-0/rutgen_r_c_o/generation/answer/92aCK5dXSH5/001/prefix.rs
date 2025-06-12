// Answer 0

#[test]
fn test_rebuild_vec_basic() {
    let mut vec = vec![1, 2, 3, 4, 5];
    let ptr = vec.as_mut_ptr();
    let len = 5;
    let cap = 5;
    let off = 0;
    unsafe {
        let rebuilt = rebuild_vec(ptr, len, cap, off);
    }
}

#[test]
fn test_rebuild_vec_with_offset() {
    let mut vec = vec![10, 20, 30, 40];
    let ptr = vec.as_mut_ptr();
    let len = 4;
    let cap = 4;
    let off = 2;
    unsafe {
        let rebuilt = rebuild_vec(ptr, len, cap, off);
    }
}

#[test]
fn test_rebuild_vec_zero_length() {
    let vec: Vec<u8> = Vec::new();
    let ptr = vec.as_mut_ptr();
    let len = 0;
    let cap = 0;
    let off = 0;
    unsafe {
        let rebuilt = rebuild_vec(ptr, len, cap, off);
    }
}

#[test]
fn test_rebuild_vec_with_full_capacity() {
    let mut vec = vec![100; 10];
    let ptr = vec.as_mut_ptr();
    let len = 10;
    let cap = 10;
    let off = 5;
    unsafe {
        let rebuilt = rebuild_vec(ptr, len, cap, off);
    }
}

#[should_panic]
fn test_rebuild_vec_off_beyond_len() {
    let mut vec = vec![1, 2, 3];
    let ptr = vec.as_mut_ptr();
    let len = 3;
    let cap = 3;
    let off = 4; // invalid offset beyond length
    unsafe {
        let rebuilt = rebuild_vec(ptr, len, cap, off);
    }
}

#[test]
fn test_rebuild_vec_large_capacity() {
    let mut vec = vec![0u8; usize::MAX / 2];
    let ptr = vec.as_mut_ptr();
    let len = usize::MAX / 2;
    let cap = usize::MAX / 2;
    let off = 0;
    unsafe {
        let rebuilt = rebuild_vec(ptr, len, cap, off);
    }
}

