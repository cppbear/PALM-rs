// Answer 0

#[test]
fn test_bucket_valid_index() {
    struct RawTable {
        buckets: usize,
        data: [usize; 4], // example data; can be adjusted based on your needs
    }

    impl RawTable {
        fn buckets(&self) -> usize {
            self.buckets
        }

        fn data_end(&self) -> *const usize {
            self.data.as_ptr().add(self.buckets)
        }
    }

    struct Bucket {
        ptr: *const usize,
    }

    impl Bucket {
        fn from_base_index(base: *const usize, index: usize) -> Self {
            Bucket {
                ptr: unsafe { base.add(index) },
            }
        }
    }

    let table = RawTable {
        buckets: 4,
        data: [1, 2, 3, 4],
    };

    unsafe {
        let bucket = table.bucket(2);
        assert_eq!(unsafe { *bucket.ptr }, 3);
    }
}

#[test]
#[should_panic]
fn test_bucket_index_out_of_bounds() {
    struct RawTable {
        buckets: usize,
        data: [usize; 4],
    }

    impl RawTable {
        fn buckets(&self) -> usize {
            self.buckets
        }

        fn data_end(&self) -> *const usize {
            self.data.as_ptr().add(self.buckets)
        }
    }

    struct Bucket {
        ptr: *const usize,
    }

    impl Bucket {
        fn from_base_index(base: *const usize, index: usize) -> Self {
            Bucket {
                ptr: unsafe { base.add(index) },
            }
        }
    }

    let table = RawTable {
        buckets: 4,
        data: [1, 2, 3, 4],
    };

    unsafe {
        let _bucket = table.bucket(4); // Out of bounds
    }
}

#[test]
fn test_bucket_zero_index() {
    struct RawTable {
        buckets: usize,
        data: [usize; 4],
    }

    impl RawTable {
        fn buckets(&self) -> usize {
            self.buckets
        }

        fn data_end(&self) -> *const usize {
            self.data.as_ptr().add(self.buckets)
        }
    }

    struct Bucket {
        ptr: *const usize,
    }

    impl Bucket {
        fn from_base_index(base: *const usize, index: usize) -> Self {
            Bucket {
                ptr: unsafe { base.add(index) },
            }
        }
    }

    let table = RawTable {
        buckets: 4,
        data: [1, 2, 3, 4],
    };

    unsafe {
        let bucket = table.bucket(0);
        assert_eq!(unsafe { *bucket.ptr }, 1);
    }
}

#[test]
fn test_bucket_max_index() {
    struct RawTable {
        buckets: usize,
        data: [usize; 4],
    }

    impl RawTable {
        fn buckets(&self) -> usize {
            self.buckets
        }

        fn data_end(&self) -> *const usize {
            self.data.as_ptr().add(self.buckets)
        }
    }

    struct Bucket {
        ptr: *const usize,
    }

    impl Bucket {
        fn from_base_index(base: *const usize, index: usize) -> Self {
            Bucket {
                ptr: unsafe { base.add(index) },
            }
        }
    }

    let table = RawTable {
        buckets: 4,
        data: [1, 2, 3, 4],
    };

    unsafe {
        let bucket = table.bucket(3);
        assert_eq!(unsafe { *bucket.ptr }, 4);
    }
}

