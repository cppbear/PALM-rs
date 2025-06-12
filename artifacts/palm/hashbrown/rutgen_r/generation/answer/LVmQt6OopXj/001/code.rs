// Answer 0


struct HashMap {
    bucket_mask: usize,
}

impl HashMap {
    fn buckets(&self) -> usize {
        self.bucket_mask + 1
    }
}

#[test]
fn test_buckets_with_zero_bucket_mask() {
    let map = HashMap { bucket_mask: 0 };
    assert_eq!(map.buckets(), 1);
}

#[test]
fn test_buckets_with_small_bucket_mask() {
    let map = HashMap { bucket_mask: 1 };
    assert_eq!(map.buckets(), 2);
}

#[test]
fn test_buckets_with_large_bucket_mask() {
    let map = HashMap { bucket_mask: 100 };
    assert_eq!(map.buckets(), 101);
}

#[test]
fn test_buckets_with_max_usize() {
    let map = HashMap { bucket_mask: std::usize::MAX };
    assert_eq!(map.buckets(), std::usize::MAX + 1);
}


