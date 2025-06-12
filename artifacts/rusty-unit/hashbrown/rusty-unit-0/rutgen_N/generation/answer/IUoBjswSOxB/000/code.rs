// Answer 0

#[derive(Debug)]
struct Bucket<T> {
    value: T,
}

struct RawTableInner<T> {
    buckets: Vec<Bucket<T>>,
}

impl<T> RawTableInner<T> {
    pub fn buckets(&self) -> usize {
        self.buckets.len()
    }

    pub fn bucket(&self, index: usize) -> &Bucket<T> {
        &self.buckets[index]
    }

    pub fn find_inner<F>(&self, hash: u64, eq: &mut F) -> Option<usize>
    where
        F: FnMut(usize) -> bool,
    {
        for (index, bucket) in self.buckets.iter().enumerate() {
            if eq(index) {
                return Some(index);
            }
        }
        None
    }
}

struct RawTable<T> {
    table: RawTableInner<T>,
}

impl<T> RawTable<T> {
    pub fn find(&self, hash: u64, mut eq: impl FnMut(&T) -> bool) -> Option<&Bucket<T>> {
        unsafe {
            let result = self
                .table
                .find_inner(hash, &mut |index| eq(self.table.bucket(index).as_ref()));

            match result {
                Some(index) => Some(self.table.bucket(index)),
                None => None,
            }
        }
    }
}

#[test]
fn test_find_existing_element() {
    let bucket1 = Bucket { value: 1 };
    let bucket2 = Bucket { value: 2 };
    let raw_table = RawTable {
        table: RawTableInner {
            buckets: vec![bucket1, bucket2],
        },
    };

    let result = raw_table.find(42, |&x| x == 2);
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, 2);
}

#[test]
fn test_find_non_existing_element() {
    let bucket1 = Bucket { value: 1 };
    let bucket2 = Bucket { value: 2 };
    let raw_table = RawTable {
        table: RawTableInner {
            buckets: vec![bucket1, bucket2],
        },
    };

    let result = raw_table.find(42, |&x| x == 3);
    assert!(result.is_none());
}

#[test]
fn test_find_empty_table() {
    let raw_table = RawTable {
        table: RawTableInner {
            buckets: vec![],
        },
    };

    let result = raw_table.find(42, |&x| x == 1);
    assert!(result.is_none());
}

