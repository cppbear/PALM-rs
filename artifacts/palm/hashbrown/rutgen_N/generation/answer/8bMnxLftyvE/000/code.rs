// Answer 0

#[derive(Default)]
struct BucketControl {
    full: bool,
}

impl BucketControl {
    fn new(full: bool) -> Self {
        BucketControl { full }
    }

    fn is_full(&self) -> bool {
        self.full
    }
}

struct HashMap {
    controls: Vec<BucketControl>,
}

impl HashMap {
    fn new(size: usize) -> Self {
        HashMap {
            controls: vec![BucketControl::default(); size],
        }
    }

    fn buckets(&self) -> usize {
        self.controls.len()
    }

    unsafe fn ctrl(&self, index: usize) -> &BucketControl {
        &self.controls[index]
    }
}

#[test]
fn test_is_bucket_full() {
    let map = HashMap::new(5);
    let index = 0;

    unsafe {
        assert!(!map.is_bucket_full(index), "Expected bucket not to be full.");

        // Set the first bucket as full
        map.controls[index] = BucketControl::new(true);
        assert!(map.is_bucket_full(index), "Expected bucket to be full.");
    }
}

#[test]
fn test_is_bucket_full_boundary_conditions() {
    let map = HashMap::new(1);
    let index = 0;

    unsafe {
        assert!(!map.is_bucket_full(index), "Expected bucket not to be full.");

        // Set the bucket as full
        map.controls[index] = BucketControl::new(true);
        assert!(map.is_bucket_full(index), "Expected bucket to be full.");
    }
}

