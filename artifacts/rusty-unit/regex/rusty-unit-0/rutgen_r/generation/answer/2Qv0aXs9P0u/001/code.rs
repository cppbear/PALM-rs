// Answer 0

#[derive(Default)]
struct SuffixCacheEntry;

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
fn test_suffix_cache_size_zero() {
    let cache = SuffixCache::new(0);
    assert_eq!(cache.table.len(), 0);
    assert_eq!(cache.version, 0);
}

#[test]
fn test_suffix_cache_size_one() {
    let cache = SuffixCache::new(1);
    assert_eq!(cache.table.len(), 1);
    assert_eq!(cache.version, 0);
}

#[test]
fn test_suffix_cache_size_large() {
    let size = 1000;
    let cache = SuffixCache::new(size);
    assert_eq!(cache.table.len(), size);
    assert_eq!(cache.version, 0);
}

#[test]
fn test_suffix_cache_size_max() {
    let size = usize::MAX;
    let cache = SuffixCache::new(size);
    assert_eq!(cache.table.len(), size);
    assert_eq!(cache.version, 0);
}

