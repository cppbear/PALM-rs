// Answer 0

#[test]
#[should_panic]
fn test_promote_to_shared_ref_count_2() {
    let mut bytes_mut = unsafe { BytesMut::with_capacity(20) };
    let ref_cnt = 2; // violating the constraint

    unsafe {
        bytes_mut.promote_to_shared(ref_cnt);
    }
}

#[test]
fn test_promote_to_shared_valid_scenario() {
    let mut bytes_mut = unsafe { BytesMut::with_capacity(20) };
    let ref_cnt = 1; // satisfying the constraint

    unsafe {
        bytes_mut.promote_to_shared(ref_cnt);
        // Verify that the pointer and values are as expected
        assert_eq!(bytes_mut.kind(), KIND_ARC); // After promotion, kind should be KIND_ARC
    }
}

#[test]
fn test_promote_to_shared_verify_contents() {
    // Setup BytesMut with unique content
    let mut bytes_mut = unsafe { BytesMut::with_capacity(20) };
    let ref_cnt = 1; // satisfying the constraint

    // Fill the BytesMut in a valid manner, ensuring it meets the constraints
    unsafe {
        bytes_mut.resize(10, 1);
        bytes_mut.promote_to_shared(ref_cnt);
    }

    unsafe {
        // Check if the contents are still the same after promotion
        assert_eq!(bytes_mut.len(), 10);
        let slice = bytes_mut.as_slice();
        assert!(slice.iter().all(|&x| x == 1)); // contents should be `1`
    }
}

