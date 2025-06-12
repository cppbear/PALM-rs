// Answer 0

#[test]
fn test_promote_to_shared_ref_count_one() {
    // Helper structure to create a BytesMut instance
    struct BytesMutTest {
        bytes_mut: BytesMut,
    }

    impl BytesMutTest {
        fn new_with_capacity(capacity: usize) -> BytesMutTest {
            let bytes_mut = unsafe { BytesMut::with_capacity(capacity) };
            BytesMutTest { bytes_mut }
        }

        fn set_data(&mut self, data: &[u8]) {
            unsafe {
                self.bytes_mut.extend_from_slice(data);
            }
        }

        fn promote(&mut self, ref_cnt: usize) {
            unsafe {
                self.bytes_mut.promote_to_shared(ref_cnt);
            }
        }
    }

    let mut bytes = BytesMutTest::new_with_capacity(10);
    bytes.set_data(&[1, 2, 3, 4, 5]);

    // Promote with ref_count = 1
    bytes.promote(1);

    // Check that the internal state is consistent
    assert_eq!(bytes.bytes_mut.kind(), KIND_ARC);
}

#[test]
fn test_promote_to_shared_ref_count_two() {
    // Helper structure to create a BytesMut instance
    struct BytesMutTest {
        bytes_mut: BytesMut,
    }

    impl BytesMutTest {
        fn new_with_capacity(capacity: usize) -> BytesMutTest {
            let bytes_mut = unsafe { BytesMut::with_capacity(capacity) };
            BytesMutTest { bytes_mut }
        }

        fn set_data(&mut self, data: &[u8]) {
            unsafe {
                self.bytes_mut.extend_from_slice(data);
            }
        }

        fn promote(&mut self, ref_cnt: usize) {
            unsafe {
                self.bytes_mut.promote_to_shared(ref_cnt);
            }
        }
    }

    let mut bytes = BytesMutTest::new_with_capacity(10);
    bytes.set_data(&[6, 7, 8, 9, 10]);

    // Promote with ref_count = 2
    bytes.promote(2);

    // Check that the internal state is consistent
    assert_eq!(bytes.bytes_mut.kind(), KIND_ARC);
}

