// Answer 0

#[test]
fn test_bucket_valid_index() {
    struct RawTableInner {
        bucket_mask: usize,
        data: Vec<i32>,
    }

    impl RawTableInner {
        fn buckets(&self) -> usize {
            self.bucket_mask + 1
        }

        fn data_end(&self) -> *const i32 {
            self.data.as_ptr().add(self.buckets())
        }
    }

    struct Bucket<T> {
        ptr: *const T,
    }

    impl<T> Bucket<T> {
        fn from_base_index(ptr: *const T, index: usize) -> Self {
            unsafe { Bucket { ptr: ptr.add(index) } }
        }

        fn as_ptr(&self) -> *const T {
            self.ptr
        }
    }

    let raw_table = RawTableInner {
        bucket_mask: 3, // meaning 4 buckets (0 to 3)
        data: vec![10, 20, 30, 40],
    };

    let index = 2; // Valid index
    let bucket = unsafe { raw_table.bucket(index) };
    assert_eq!(unsafe { *bucket.as_ptr() }, 30);
}

#[test]
#[should_panic]
fn test_bucket_index_out_of_bounds() {
    struct RawTableInner {
        bucket_mask: usize,
        data: Vec<i32>,
    }

    impl RawTableInner {
        fn buckets(&self) -> usize {
            self.bucket_mask + 1
        }

        fn data_end(&self) -> *const i32 {
            self.data.as_ptr().add(self.buckets())
        }
    }

    struct Bucket<T> {
        ptr: *const T,
    }

    impl<T> Bucket<T> {
        fn from_base_index(ptr: *const T, index: usize) -> Self {
            unsafe { Bucket { ptr: ptr.add(index) } }
        }

        fn as_ptr(&self) -> *const T {
            self.ptr
        }
    }

    let raw_table = RawTableInner {
        bucket_mask: 3,
        data: vec![10, 20, 30, 40],
    };

    let index = 4; // Invalid index, should panic
    let _bucket = unsafe { raw_table.bucket(index) };
}

