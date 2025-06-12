// Answer 0

#[derive(Default)]
struct SuffixCacheEntry {
    // Assuming some fields here for demonstration
}

struct SuffixCache {
    table: Vec<SuffixCacheEntry>,
    version: usize,
}

impl SuffixCache {
    fn new(size: usize) -> Self {
        SuffixCache {
            table: vec![SuffixCacheEntry::default(); size],
            version: 0,
        }
    }
}

#[test]
fn test_suffix_cache_new_zero_size() {
    let cache = SuffixCache::new(0);
    assert_eq!(cache.table.len(), 0);
    assert_eq!(cache.version, 0);
}

#[test]
fn test_suffix_cache_new_non_zero_size() {
    let size = 10;
    let cache = SuffixCache::new(size);
    assert_eq!(cache.table.len(), size);
    assert_eq!(cache.version, 0);
}

#[test]
fn test_suffix_cache_new_large_size() {
    let size = 1000;
    let cache = SuffixCache::new(size);
    assert_eq!(cache.table.len(), size);
    assert_eq!(cache.version, 0);
}

