// Answer 0

#[derive(Debug)]
struct Bucket<T> {
    value: T,
}

struct Slice<T> {
    buckets: Vec<Bucket<T>>,
}

impl<T> Slice<T> {
    pub(super) const fn from_slice(entries: &[Bucket<T>]) -> &Self {
        unsafe { &*(entries as *const [Bucket<T>] as *const Self) }
    }
}

#[test]
fn test_from_slice_valid() {
    let bucket1 = Bucket { value: 1 };
    let bucket2 = Bucket { value: 2 };
    let entries = &[bucket1, bucket2];
    let slice = Slice::from_slice(entries);
    assert_eq!(slice.buckets.len(), 2);
}

#[test]
#[should_panic]
fn test_from_slice_empty() {
    let entries: &[Bucket<i32>] = &[];
    let _slice = Slice::from_slice(entries);
}

#[test]
fn test_from_slice_single_element() {
    let single_bucket = Bucket { value: 42 };
    let entries = &[single_bucket];
    let slice = Slice::from_slice(entries);
    assert_eq!(slice.buckets.len(), 1);
}

#[test]
#[should_panic]
fn test_from_slice_invalid_memory() {
    let mut buckets = vec![Bucket { value: 10 }];
    let ptr = buckets.as_mut_ptr();
    std::mem::forget(buckets);
    let slice: &Slice<i32> = unsafe { Slice::from_slice(std::slice::from_raw_parts(ptr, 1)) };
    drop(slice);
}

