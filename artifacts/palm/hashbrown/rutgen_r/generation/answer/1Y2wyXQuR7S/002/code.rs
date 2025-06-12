// Answer 0

#[test]
fn test_bucket_ptr_valid() {
    struct RawTableInner {
        bucket_mask: usize,
        data: Vec<u8>,
    }

    impl RawTableInner {
        fn new(size: usize) -> Self {
            Self {
                bucket_mask: size.next_power_of_two() - 1,
                data: vec![0; size * std::mem::size_of::<u8>()],
            }
        }

        fn buckets(&self) -> usize {
            self.bucket_mask + 1
        }

        fn data_end(&self) -> &Vec<u8> {
            &self.data
        }

        unsafe fn bucket_ptr(&self, index: usize, size_of: usize) -> *mut u8 {
            debug_assert_ne!(self.bucket_mask, 0);
            debug_assert!(index < self.buckets());
            let base: *mut u8 = self.data_end().as_ptr();
            base.sub((index + 1) * size_of)
        }
    }

    let table = RawTableInner::new(4); // 4 buckets
    let index = 2;
    let size_of = std::mem::size_of::<u8>();
    let ptr = unsafe { table.bucket_ptr(index, size_of) };

    assert_eq!(ptr as usize, table.data_end().as_ptr() as usize - ((index + 1) * size_of));
}

#[test]
#[should_panic(expected = "assertion failed: index < self.buckets()")]
fn test_bucket_ptr_index_out_of_bounds() {
    struct RawTableInner {
        bucket_mask: usize,
        data: Vec<u8>,
    }

    impl RawTableInner {
        fn new(size: usize) -> Self {
            Self {
                bucket_mask: size.next_power_of_two() - 1,
                data: vec![0; size * std::mem::size_of::<u8>()],
            }
        }

        fn buckets(&self) -> usize {
            self.bucket_mask + 1
        }

        fn data_end(&self) -> &Vec<u8> {
            &self.data
        }

        unsafe fn bucket_ptr(&self, index: usize, size_of: usize) -> *mut u8 {
            debug_assert_ne!(self.bucket_mask, 0);
            debug_assert!(index < self.buckets());
            let base: *mut u8 = self.data_end().as_ptr();
            base.sub((index + 1) * size_of)
        }
    }

    let table = RawTableInner::new(4); // 4 buckets
    let index = 4; // Out of bounds
    let size_of = std::mem::size_of::<u8>();
    unsafe { table.bucket_ptr(index, size_of) };
}

#[test]
#[should_panic(expected = "assertion failed: self.bucket_mask != 0")]
fn test_bucket_ptr_zero_bucket_mask() {
    struct RawTableInner {
        bucket_mask: usize,
        data: Vec<u8>,
    }

    impl RawTableInner {
        fn new(size: usize) -> Self {
            Self {
                bucket_mask: 0, // Invalid bucket mask to trigger assertion
                data: vec![0; size * std::mem::size_of::<u8>()],
            }
        }

        fn buckets(&self) -> usize {
            self.bucket_mask + 1
        }

        fn data_end(&self) -> &Vec<u8> {
            &self.data
        }

        unsafe fn bucket_ptr(&self, index: usize, size_of: usize) -> *mut u8 {
            debug_assert_ne!(self.bucket_mask, 0);
            debug_assert!(index < self.buckets());
            let base: *mut u8 = self.data_end().as_ptr();
            base.sub((index + 1) * size_of)
        }
    }

    let table = RawTableInner::new(4); // 4 buckets
    let index = 0; 
    let size_of = std::mem::size_of::<u8>();
    unsafe { table.bucket_ptr(index, size_of) };
}

