// Answer 0

#[derive(Debug)]
struct SuffixCacheKey {
    from_inst: u32,
    start: u32,
    end: u32,
}

struct HashTable {
    table: Vec<usize>,
}

impl HashTable {
    fn new(size: usize) -> Self {
        HashTable {
            table: vec![0; size],
        }
    }

    fn hash(&self, suffix: &SuffixCacheKey) -> usize {
        const FNV_PRIME: u64 = 1099511628211;
        let mut h = 14695981039346656037;
        h = (h ^ (suffix.from_inst as u64)).wrapping_mul(FNV_PRIME);
        h = (h ^ (suffix.start as u64)).wrapping_mul(FNV_PRIME);
        h = (h ^ (suffix.end as u64)).wrapping_mul(FNV_PRIME);
        (h as usize) % self.table.len()
    }
}

#[test]
fn test_hash_basic_case() {
    let table = HashTable::new(10);
    let suffix = SuffixCacheKey { from_inst: 1, start: 2, end: 3 };
    let result = table.hash(&suffix);
    assert!(result < table.table.len());
}

#[test]
fn test_hash_with_different_suffixes() {
    let table = HashTable::new(10);
    let suffix1 = SuffixCacheKey { from_inst: 5, start: 10, end: 15 };
    let suffix2 = SuffixCacheKey { from_inst: 3, start: 4, end: 5 };
    let result1 = table.hash(&suffix1);
    let result2 = table.hash(&suffix2);
    assert!(result1 < table.table.len());
    assert!(result2 < table.table.len());
}

#[test]
fn test_hash_boundary_case() {
    let table = HashTable::new(1);
    let suffix = SuffixCacheKey { from_inst: u32::MAX, start: u32::MAX, end: u32::MAX };
    let result = table.hash(&suffix);
    assert_eq!(result, 0);
}

