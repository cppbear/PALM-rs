// Answer 0

#[test]
fn test_borrow_mut_valid() {
    let mut bytes_mut = BytesMut {
        ptr: NonNull::new(unsafe { &mut *(Vec::from_iter(vec![1u8, 2, 3]).as_mut_ptr()) }).unwrap(),
        len: 3,
        cap: 4,
        data: std::ptr::null_mut(),
    };

    let borrowed: &mut [u8] = bytes_mut.borrow_mut();
    assert_eq!(borrowed.len(), 3);
    assert_eq!(borrowed[0], 1);
}

#[test]
#[should_panic]
fn test_borrow_mut_panic_empty() {
    let mut bytes_mut = BytesMut {
        ptr: NonNull::new(std::ptr::null_mut()).unwrap(),
        len: 0,
        cap: 0,
        data: std::ptr::null_mut(),
    };

    let _borrowed: &mut [u8] = bytes_mut.borrow_mut();
} 

#[test]
fn test_borrow_mut_edge_case() {
    let mut bytes_mut = BytesMut {
        ptr: NonNull::new(unsafe { &mut *(Vec::from_iter(vec![0u8; 8]).as_mut_ptr()) }).unwrap(),
        len: 8,
        cap: 8,
        data: std::ptr::null_mut(),
    };

    let borrowed: &mut [u8] = bytes_mut.borrow_mut();
    assert_eq!(borrowed.len(), 8);
    for i in 0..borrowed.len() {
        borrowed[i] = (i + 1) as u8;
    }
    assert_eq!(borrowed[0], 1);
    assert_eq!(borrowed[7], 8);
}

