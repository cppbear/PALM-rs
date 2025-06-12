// Answer 0

#[test]
fn test_from_boxed() {
    struct Bucket<T> {
        value: T,
    }

    struct Slice {
        buckets: Box<[Bucket<i32>]>,
    }

    impl Slice {
        pub(super) fn from_boxed(entries: Box<[Bucket<i32>]>) -> Box<Self> {
            unsafe { Box::from_raw(Box::into_raw(entries) as *mut Self) }
        }
    }

    let entries = Box::new([
        Bucket { value: 1 },
        Bucket { value: 2 },
        Bucket { value: 3 },
    ]);
    let slice = Slice::from_boxed(entries);
    
    assert_eq!(slice.buckets.len(), 3);
    assert_eq!(slice.buckets[0].value, 1);
    assert_eq!(slice.buckets[1].value, 2);
    assert_eq!(slice.buckets[2].value, 3);
}

#[should_panic]
fn test_from_boxed_empty() {
    struct Bucket<T> {
        value: T,
    }

    struct Slice {
        buckets: Box<[Bucket<i32>]>,
    }

    impl Slice {
        pub(super) fn from_boxed(entries: Box<[Bucket<i32>]>) -> Box<Self> {
            unsafe { Box::from_raw(Box::into_raw(entries) as *mut Self) }
        }
    }

    let entries: Box<[Bucket<i32>]> = Box::new([]);
    let _slice = Slice::from_boxed(entries);
}

