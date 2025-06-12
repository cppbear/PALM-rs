// Answer 0

#[test]
fn test_rebuild_vec_basic() {
    let v = vec![1u8, 2, 3, 4, 5];
    let ptr = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();
    let off = 2;

    unsafe {
        let rebuilt_vec = rebuild_vec(ptr, len, cap, off);
        assert_eq!(rebuilt_vec.len(), len + off);
        assert_eq!(rebuilt_vec.capacity(), cap + off);
        assert_eq!(&*rebuilt_vec, &[0, 0, 1, 2, 3, 4, 5]);
    }
}

#[test]
fn test_rebuild_vec_zero_offset() {
    let v = vec![10u8, 20, 30];
    let ptr = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();
    let off = 0;

    unsafe {
        let rebuilt_vec = rebuild_vec(ptr, len, cap, off);
        assert_eq!(rebuilt_vec.len(), len);
        assert_eq!(rebuilt_vec.capacity(), cap);
        assert_eq!(&*rebuilt_vec, &[10, 20, 30]);
    }
}

#[test]
fn test_rebuild_vec_large_offset() {
    let v = vec![100u8, 200, 300, 400];
    let ptr = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();
    let off = 3;

    unsafe {
        let rebuilt_vec = rebuild_vec(ptr, len, cap, off);
        assert_eq!(rebuilt_vec.len(), len + off);
        assert_eq!(rebuilt_vec.capacity(), cap + off);
        assert_eq!(&*rebuilt_vec, &[0, 0, 0, 100, 200, 300, 400]);
    }
}

#[should_panic]
#[test]
fn test_rebuild_vec_invalid_pointer() {
    let len = 5;
    let cap = 5;
    let off = 10; // Offset larger than the allocated size
    let invalid_ptr = std::ptr::null_mut::<u8>();

    unsafe {
        rebuild_vec(invalid_ptr, len, cap, off);
    }
}

