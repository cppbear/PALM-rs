// Answer 0

#[test]
fn test_bucket_valid_index() {
    struct RawTableInner {
        bucket_mask: usize,
        buckets: usize,
    }

    impl RawTableInner {
        fn buckets(&self) -> usize {
            self.buckets
        }

        fn data_end<T>(&self) -> *mut T {
            std::ptr::null_mut() 
        }
    }

    struct Bucket<T> {
        _marker: std::marker::PhantomData<T>,
    }

    impl<T> Bucket<T> {
        fn from_base_index(_: *mut T, _: usize) -> Self {
            Bucket { _marker: std::marker::PhantomData }
        }
    }

    let table = RawTableInner { bucket_mask: 1, buckets: 5 };
    let index = 2;

    unsafe {
        let bucket = table.bucket::<i32>(index);
        assert!(!std::ptr::null_mut::<i32>() == bucket.from_base_index(std::ptr::null_mut(), index));
    }
}

#[test]
#[should_panic]
fn test_bucket_out_of_bounds() {
    struct RawTableInner {
        bucket_mask: usize,
        buckets: usize,
    }

    impl RawTableInner {
        fn buckets(&self) -> usize {
            self.buckets
        }

        fn data_end<T>(&self) -> *mut T {
            std::ptr::null_mut() 
        }
    }

    struct Bucket<T> {
        _marker: std::marker::PhantomData<T>,
    }

    impl<T> Bucket<T> {
        fn from_base_index(_: *mut T, _: usize) -> Self {
            Bucket { _marker: std::marker::PhantomData }
        }
    }

    let table = RawTableInner { bucket_mask: 1, buckets: 5 };
    let index = 5;

    unsafe {
        let _bucket = table.bucket::<i32>(index); // This should panic
    }
}

#[test]
fn test_bucket_minimum_index() {
    struct RawTableInner {
        bucket_mask: usize,
        buckets: usize,
    }

    impl RawTableInner {
        fn buckets(&self) -> usize {
            self.buckets
        }

        fn data_end<T>(&self) -> *mut T {
            std::ptr::null_mut() 
        }
    }

    struct Bucket<T> {
        _marker: std::marker::PhantomData<T>,
    }

    impl<T> Bucket<T> {
        fn from_base_index(_: *mut T, _: usize) -> Self {
            Bucket { _marker: std::marker::PhantomData }
        }
    }

    let table = RawTableInner { bucket_mask: 1, buckets: 5 };
    let index = 0;

    unsafe {
        let bucket = table.bucket::<i32>(index);
        assert!(!std::ptr::null_mut::<i32>() == bucket.from_base_index(std::ptr::null_mut(), index));
    }
}

#[test]
fn test_bucket_zero_size_type() {
    struct RawTableInner {
        bucket_mask: usize,
        buckets: usize,
    }

    impl RawTableInner {
        fn buckets(&self) -> usize {
            self.buckets
        }
    }

    struct Bucket<T> {
        _marker: std::marker::PhantomData<T>,
    }

    impl<T> Bucket<T> {
        fn from_base_index(_: *mut T, _: usize) -> Self {
            Bucket { _marker: std::marker::PhantomData }
        }
    }

    let table = RawTableInner { bucket_mask: 1, buckets: 5 };
    let index = 4;

    unsafe {
        let bucket = table.bucket::<()>(index); // Using zero-size type
        assert!(!std::ptr::null_mut::<()>() == bucket.from_base_index(std::ptr::null_mut(), index));
    }
}

