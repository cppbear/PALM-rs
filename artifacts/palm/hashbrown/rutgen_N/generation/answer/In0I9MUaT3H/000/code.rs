// Answer 0

#[derive(Default)]
struct Bucket<T> {
    data: Vec<T>,
}

impl<T> Bucket<T> {
    pub fn to_base_index(&self, data_end: usize) -> usize {
        self.data.len().min(data_end)
    }
}

struct Hashbrown<T> {
    buckets: Vec<Bucket<T>>,
}

impl<T> Hashbrown<T> {
    pub fn data_end(&self) -> usize {
        self.buckets.len()
    }

    pub unsafe fn bucket_index(&self, bucket: &Bucket<T>) -> usize {
        bucket.to_base_index(self.data_end())
    }
}

#[test]
fn test_bucket_index_with_empty_bucket() {
    let hashbrown: Hashbrown<i32> = Hashbrown { buckets: vec![Bucket { data: Vec::new() }] };
    let bucket = Bucket { data: Vec::new() };
    let index = unsafe { hashbrown.bucket_index(&bucket) };
    assert_eq!(index, 0);
}

#[test]
fn test_bucket_index_with_non_empty_bucket() {
    let hashbrown: Hashbrown<i32> = Hashbrown { buckets: vec![Bucket { data: vec![1, 2, 3] }] };
    let bucket = Bucket { data: vec![4, 5] };
    let index = unsafe { hashbrown.bucket_index(&bucket) };
    assert_eq!(index, 2);
}

#[test]
fn test_bucket_index_with_boundary_conditions() {
    let hashbrown: Hashbrown<i32> = Hashbrown { buckets: vec![Bucket { data: vec![1, 2, 3] }] };
    let bucket = Bucket { data: vec![1, 2, 3, 4, 5, 6] };
    let index = unsafe { hashbrown.bucket_index(&bucket) };
    assert_eq!(index, 3);
}

