// Answer 0

#[test]
fn test_bucket_ptr_valid() {
    struct RawTableInner {
        bucket_mask: usize,
        data: Vec<u8>,
    }

    impl RawTableInner {
        fn buckets(&self) -> usize {
            self.data.len() // For simplicity, let's assume this returns the length of the data.
        }

        fn data_end(&self) -> &Vec<u8> {
            &self.data
        }
    }

    let table = RawTableInner {
        bucket_mask: 1, // Not zero, valid for testing
        data: vec![1, 2, 3, 4], // Simulating data storage
    };

    unsafe {
        let ptr = table.bucket_ptr(2, std::mem::size_of::<u8>());
        assert_eq!(*ptr, 3); // Should point to the third element, which is `3`.
    }
}

#[test]
#[should_panic]
fn test_bucket_ptr_out_of_bounds() {
    struct RawTableInner {
        bucket_mask: usize,
        data: Vec<u8>,
    }

    impl RawTableInner {
        fn buckets(&self) -> usize {
            self.data.len()
        }

        fn data_end(&self) -> &Vec<u8> {
            &self.data
        }
    }

    let table = RawTableInner {
        bucket_mask: 1,
        data: vec![1, 2, 3, 4],
    };

    unsafe {
        let _ptr = table.bucket_ptr(4, std::mem::size_of::<u8>()); // Index 4 is out of bounds (length is 4).
    }
}

#[test]
#[should_panic]
fn test_bucket_ptr_with_zero_size() {
    struct RawTableInner {
        bucket_mask: usize,
        data: Vec<u8>,
    }

    impl RawTableInner {
        fn buckets(&self) -> usize {
            self.data.len()
        }

        fn data_end(&self) -> &Vec<u8> {
            &self.data
        }
    }

    let table = RawTableInner {
        bucket_mask: 1,
        data: vec![1, 2, 3, 4],
    };

    unsafe {
        let _ptr = table.bucket_ptr(1, 0); // Size of zero is invalid.
    }
}

