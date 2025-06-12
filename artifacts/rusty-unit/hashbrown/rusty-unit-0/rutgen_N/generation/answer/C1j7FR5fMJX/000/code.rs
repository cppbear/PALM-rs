// Answer 0

#[test]
fn test_bucket_valid_index() {
    struct RawTable {
        data_end: *const i32,
        buckets: usize,
    }

    impl RawTable {
        fn buckets(&self) -> usize {
            self.buckets
        }

        unsafe fn data_end(&self) -> *const i32 {
            self.data_end
        }

        unsafe fn bucket(&self, index: usize) -> *const i32 {
            debug_assert_ne!(self.buckets(), 0);
            debug_assert!(index < self.buckets());
            self.data_end().offset(index as isize)
        }
    }

    let data: [i32; 5] = [1, 2, 3, 4, 5];
    let raw_table = RawTable {
        data_end: data.as_ptr().add(data.len()), // Pointer to end of data
        buckets: data.len(), // Number of buckets
    };

    unsafe {
        let bucket_ptr = raw_table.bucket(2);
        assert_eq!(*bucket_ptr, 4); // Accessing the value at index 2, which should be 4
    }
}

#[test]
fn test_bucket_out_of_bounds() {
    struct RawTable {
        data_end: *const i32,
        buckets: usize,
    }

    impl RawTable {
        fn buckets(&self) -> usize {
            self.buckets
        }

        unsafe fn data_end(&self) -> *const i32 {
            self.data_end
        }

        unsafe fn bucket(&self, index: usize) -> *const i32 {
            debug_assert_ne!(self.buckets(), 0);
            debug_assert!(index < self.buckets());
            self.data_end().offset(index as isize)
        }
    }

    let data: [i32; 5] = [1, 2, 3, 4, 5];
    let raw_table = RawTable {
        data_end: data.as_ptr().add(data.len()),
        buckets: data.len(),
    };

    unsafe {
        let result = std::panic::catch_unwind(|| {
            let _ = raw_table.bucket(5); // Index out of bounds
        });
        assert!(result.is_err());
    }
}

#[test]
fn test_bucket_zero_index() {
    struct RawTable {
        data_end: *const i32,
        buckets: usize,
    }

    impl RawTable {
        fn buckets(&self) -> usize {
            self.buckets
        }

        unsafe fn data_end(&self) -> *const i32 {
            self.data_end
        }

        unsafe fn bucket(&self, index: usize) -> *const i32 {
            debug_assert_ne!(self.buckets(), 0);
            debug_assert!(index < self.buckets());
            self.data_end().offset(index as isize)
        }
    }

    let data: [i32; 5] = [1, 2, 3, 4, 5];
    let raw_table = RawTable {
        data_end: data.as_ptr().add(data.len()),
        buckets: data.len(),
    };

    unsafe {
        let bucket_ptr = raw_table.bucket(0);
        assert_eq!(*bucket_ptr, 1); // Accessing the value at index 0, which should be 1
    }
}

