// Answer 0

#[derive(Default)]
struct Table {
    buckets: Vec<Option<usize>>,
}

impl Table {
    fn is_bucket_full(&self, index: usize) -> bool {
        self.buckets.get(index).map_or(false, |bucket| bucket.is_some())
    }
}

struct HashMap {
    table: Table,
}

impl HashMap {
    fn new(size: usize) -> Self {
        HashMap {
            table: Table {
                buckets: vec![None; size],
            },
        }
    }

    unsafe fn is_bucket_full(&self, index: usize) -> bool {
        self.table.is_bucket_full(index)
    }
}

#[test]
fn test_is_bucket_full_empty() {
    let map = HashMap::new(10);
    unsafe {
        assert_eq!(map.is_bucket_full(0), false);
    }
}

#[test]
fn test_is_bucket_full_filled() {
    let mut map = HashMap::new(10);
    map.table.buckets[0] = Some(1);
    unsafe {
        assert_eq!(map.is_bucket_full(0), true);
    }
}

#[test]
fn test_is_bucket_full_out_of_bounds() {
    let map = HashMap::new(10);
    unsafe {
        let result = std::panic::catch_unwind(|| {
            map.is_bucket_full(10);
        });
        assert!(result.is_err());
    }
}

