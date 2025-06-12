// Answer 0

#[test]
fn test_bucket_ptr_valid() {
    struct RawTableInner {
        data: Vec<u8>,
        bucket_mask: usize,
    }

    impl RawTableInner {
        fn new(size: usize) -> Self {
            Self {
                data: vec![0; size * 5], // Simulating 5 elements for testing
                bucket_mask: size - 1,
            }
        }

        fn buckets(&self) -> usize {
            self.bucket_mask + 1
        }

        fn data_end(&self) -> &[u8] {
            &self.data
        }
    }

    let table = RawTableInner::new(5); // Creating a table with 5 buckets
    let size_of_element = std::mem::size_of::<u8>();
    let index = 3; // A valid index (0 to 4 for 5 buckets)

    unsafe {
        let ptr = table.bucket_ptr(index, size_of_element);
        assert_eq!(*ptr, 0); // This should point to the first element (initialized to 0)
    }
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_bucket_ptr_index_out_of_bounds() {
    struct RawTableInner {
        data: Vec<u8>,
        bucket_mask: usize,
    }

    impl RawTableInner {
        fn new(size: usize) -> Self {
            Self {
                data: vec![0; size * 5],
                bucket_mask: size - 1,
            }
        }

        fn buckets(&self) -> usize {
            self.bucket_mask + 1
        }

        fn data_end(&self) -> &[u8] {
            &self.data
        }
    }

    let table = RawTableInner::new(5); // 5 buckets
    let size_of_element = std::mem::size_of::<u8>();
    let invalid_index = 5; // Invalid index (out of bounds)

    unsafe {
        table.bucket_ptr(invalid_index, size_of_element); // This should panic
    }
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_bucket_ptr_zero_mask() {
    struct RawTableInner {
        data: Vec<u8>,
        bucket_mask: usize,
    }

    impl RawTableInner {
        fn new(size: usize) -> Self {
            Self {
                data: vec![0; size * 5],
                bucket_mask: 0, // Simulating a zero mask to trigger panic
            }
        }

        fn buckets(&self) -> usize {
            self.bucket_mask + 1
        }

        fn data_end(&self) -> &[u8] {
            &self.data
        }
    }

    let table = RawTableInner::new(5);
    let size_of_element = std::mem::size_of::<u8>();
    let index = 0; // A valid index

    unsafe {
        table.bucket_ptr(index, size_of_element); // This should panic due to zero bucket mask
    }
}

#[test]
fn test_bucket_ptr_size_of() {
    struct RawTableInner {
        data: Vec<u8>,
        bucket_mask: usize,
    }

    impl RawTableInner {
        fn new(size: usize) -> Self {
            Self {
                data: vec![0; size * 5],
                bucket_mask: size - 1,
            }
        }

        fn buckets(&self) -> usize {
            self.bucket_mask + 1
        }

        fn data_end(&self) -> &[u8] {
            &self.data
        }
    }

    let table = RawTableInner::new(5);
    let size_of_element = std::mem::size_of::<u8>();
    let index = 1; // A valid index

    unsafe {
        let ptr = table.bucket_ptr(index, size_of_element);
        assert!(ptr != std::ptr::null_mut()); // Ensure that we do not receive a null pointer
    }
}

