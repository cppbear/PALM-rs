// Answer 0

#[test]
fn test_buckets_with_zero_bucket_mask() {
    struct Table {
        bucket_mask: usize,
    }

    struct HashTable {
        table: Table,
    }

    let hash_table = HashTable { table: Table { bucket_mask: 0 } };
    assert_eq!(hash_table.buckets(), 1);
}

#[test]
fn test_buckets_with_positive_bucket_mask() {
    struct Table {
        bucket_mask: usize,
    }

    struct HashTable {
        table: Table,
    }

    let hash_table = HashTable { table: Table { bucket_mask: 5 } };
    assert_eq!(hash_table.buckets(), 6);
}

#[test]
fn test_buckets_with_large_bucket_mask() {
    struct Table {
        bucket_mask: usize,
    }

    struct HashTable {
        table: Table,
    }

    let hash_table = HashTable { table: Table { bucket_mask: usize::MAX - 1 } };
    assert_eq!(hash_table.buckets(), usize::MAX);
}

