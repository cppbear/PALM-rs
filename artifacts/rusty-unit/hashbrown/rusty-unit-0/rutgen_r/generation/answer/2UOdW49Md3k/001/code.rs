// Answer 0

#[test]
fn test_bucket_valid_case() {
    struct RawTableInner {
        bucket_mask: usize,
        data: Vec<u32>, // Example data type T
    }

    impl RawTableInner {
        fn buckets(&self) -> usize {
            self.data.len()
        }

        fn data_end<T>(&self) -> *const T {
            self.data.as_ptr() as *const T
        }

        unsafe fn bucket<T>(&self, index: usize) -> Bucket<T> {
            debug_assert_ne!(self.bucket_mask, 0);
            debug_assert!(index < self.buckets());
            Bucket::from_base_index(self.data_end::<T>(), index)
        }
    }

    struct Bucket<T> {
        ptr: *const T,
    }

    impl<T> Bucket<T> {
        fn from_base_index(base: *const T, index: usize) -> Self {
            Bucket {
                ptr: unsafe { base.add(index) }
            }
        }

        fn as_ptr(&self) -> *const T {
            self.ptr
        }
    }

    let raw_table = RawTableInner {
        bucket_mask: 1,
        data: vec![10, 20, 30, 40, 50], // Sample data
    };

    unsafe {
        let bucket = raw_table.bucket::<u32>(2);
        assert_eq!(unsafe { *bucket.as_ptr() }, 30);
    }
}

#[test]
#[should_panic]
fn test_bucket_out_of_bounds() {
    struct RawTableInner {
        bucket_mask: usize,
        data: Vec<u32>,
    }

    impl RawTableInner {
        fn buckets(&self) -> usize {
            self.data.len()
        }

        fn data_end<T>(&self) -> *const T {
            self.data.as_ptr() as *const T
        }

        unsafe fn bucket<T>(&self, index: usize) -> Bucket<T> {
            debug_assert_ne!(self.bucket_mask, 0);
            debug_assert!(index < self.buckets());
            Bucket::from_base_index(self.data_end::<T>(), index)
        }
    }

    struct Bucket<T> {
        ptr: *const T,
    }

    impl<T> Bucket<T> {
        fn from_base_index(base: *const T, index: usize) -> Self {
            Bucket {
                ptr: unsafe { base.add(index) }
            }
        }

        fn as_ptr(&self) -> *const T {
            self.ptr
        }
    }

    let raw_table = RawTableInner {
        bucket_mask: 1,
        data: vec![10, 20, 30, 40, 50],
    };

    unsafe {
        // This will panic because index 5 is out of bounds for the data vector of length 5
        let _bucket = raw_table.bucket::<u32>(5);
    }
}

#[test]
fn test_bucket_zero_index_on_unallocated() {
    struct RawTableInner {
        bucket_mask: usize,
        data: Vec<u32>,
    }

    impl RawTableInner {
        fn buckets(&self) -> usize {
            self.data.len()
        }

        fn data_end<T>(&self) -> *const T {
            self.data.as_ptr() as *const T
        }

        unsafe fn bucket<T>(&self, index: usize) -> Bucket<T> {
            debug_assert_ne!(self.bucket_mask, 0);
            debug_assert!(index < self.buckets());
            Bucket::from_base_index(self.data_end::<T>(), index)
        }
    }

    struct Bucket<T> {
        ptr: *const T,
    }

    impl<T> Bucket<T> {
        fn from_base_index(base: *const T, index: usize) -> Self {
            Bucket {
                ptr: unsafe { base.add(index) }
            }
        }

        fn as_ptr(&self) -> *const T {
            self.ptr
        }
    }

    let raw_table = RawTableInner {
        bucket_mask: 0, // Unallocated scenario
        data: vec![],
    };

    unsafe {
        let bucket = raw_table.bucket::<u32>(0);
        // This is safe but results in undefined behavior if we dereference:
        assert_eq!(bucket.as_ptr(), std::ptr::null());
    }
}

