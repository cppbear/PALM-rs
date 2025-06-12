// Answer 0

#[test]
fn test_rebuild_vec_valid() {
    let mut vec = vec![1, 2, 3, 4, 5];
    let len = vec.len();
    let cap = vec.capacity();
    
    let ptr = vec.as_mut_ptr();

    // Call rebuild_vec with valid inputs
    let rebuilt_vec = unsafe { rebuild_vec(ptr, len, cap, 0) };
    
    assert_eq!(rebuilt_vec.len(), len);
    assert_eq!(rebuilt_vec.capacity(), cap);
    assert_eq!(rebuilt_vec.as_slice(), vec.as_slice());
}

#[test]
#[should_panic]
fn test_rebuild_vec_out_of_bounds() {
    let mut vec = vec![1, 2, 3, 4, 5];
    let len = vec.len();
    let cap = vec.capacity();
    
    let ptr = vec.as_mut_ptr();

    // Call rebuild_vec with an off that exceeds the length
    let _ = unsafe { rebuild_vec(ptr, len, cap, len + 1) };
}

#[test]
fn test_rebuild_vec_zero_offset() {
    let mut vec = vec![10, 20, 30, 40, 50];
    let len = vec.len();
    let cap = vec.capacity();

    let ptr = vec.as_mut_ptr();

    // Call rebuild_vec with zero offset
    let rebuilt_vec = unsafe { rebuild_vec(ptr, len, cap, 0) };

    assert_eq!(rebuilt_vec.len(), len);
    assert_eq!(rebuilt_vec.capacity(), cap);
    assert_eq!(rebuilt_vec.as_slice(), vec.as_slice());
}

#[test]
fn test_rebuild_vec_non_zero_offset() {
    let mut vec = vec![100, 200, 300, 400, 500];
    let len = vec.len();
    let cap = vec.capacity();

    let ptr = vec.as_mut_ptr();

    // Call rebuild_vec with a valid non-zero offset
    let rebuilt_vec = unsafe { rebuild_vec(ptr, len, cap, 2) };

    assert_eq!(rebuilt_vec.len(), len + 2);
    assert_eq!(rebuilt_vec.capacity(), cap + 2);
    assert_eq!(rebuilt_vec.as_slice(), &[0, 0, 100, 200, 300, 400, 500]);
}

