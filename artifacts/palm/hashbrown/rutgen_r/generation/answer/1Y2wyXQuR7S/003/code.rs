// Answer 0

#[test]
fn test_bucket_ptr_valid() {
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
        let ptr = table.bucket_ptr(2, std::mem::size_of::<u8>());
        assert_eq!(*ptr, 3);
    }
}

#[test]
#[should_panic]
fn test_bucket_ptr_index_out_of_bounds() {
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
        let _ = table.bucket_ptr(4, std::mem::size_of::<u8>());
    }
}

#[test]
#[should_panic]
fn test_bucket_ptr_bucket_mask_zero() {
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
        bucket_mask: 0,
        data: vec![1, 2, 3, 4],
    };

    unsafe {
        let _ = table.bucket_ptr(1, std::mem::size_of::<u8>());
    }
}

