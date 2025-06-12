// Answer 0

#[test]
fn test_bucket_valid_case() {
    struct RawTable<T> {
        data: Vec<T>,
        bucket_count: usize,
    }

    impl<T> RawTable<T> {
        fn new(bucket_count: usize) -> Self {
            RawTable {
                data: Vec::with_capacity(bucket_count),
                bucket_count,
            }
        }

        fn buckets(&self) -> usize {
            self.bucket_count
        }

        // Placeholder for data_end method
        fn data_end(&self) -> *const T {
            self.data.as_ptr().add(self.data.len())
        }

        unsafe fn bucket(&self, index: usize) -> *const T {
            assert!(index < self.buckets());
            self.data_end().add(index)
        }
    }

    let table: RawTable<i32> = RawTable::new(5);
    let index = 3;
    let bucket_ptr = unsafe { table.bucket(index) };
    assert!(!bucket_ptr.is_null());
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_bucket_out_of_bounds() {
    struct RawTable<T> {
        bucket_count: usize,
    }

    impl<T> RawTable<T> {
        fn new(bucket_count: usize) -> Self {
            RawTable { bucket_count }
        }

        fn buckets(&self) -> usize {
            self.bucket_count
        }

        unsafe fn bucket(&self, index: usize) -> *const T {
            assert!(index < self.buckets());
            std::ptr::null() // Placeholder
        }
    }

    let table: RawTable<i32> = RawTable::new(5);
    let index = 5;  // This is equal to self.buckets(), should panic
    unsafe { table.bucket(index) };
}

