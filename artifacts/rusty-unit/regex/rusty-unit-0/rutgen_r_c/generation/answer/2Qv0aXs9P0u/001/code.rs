// Answer 0

#[test]
fn test_suffix_cache_new_zero_size() {
    let cache = SuffixCache::new(0);
    assert_eq!(cache.table.len(), 0);
    assert_eq!(cache.version, 0);
}

#[test]
fn test_suffix_cache_new_small_size() {
    let size = 5;
    let cache = SuffixCache::new(size);
    assert_eq!(cache.table.len(), size);
    assert_eq!(cache.version, 0);
    for entry in cache.table.iter() {
        assert_eq!(entry, &SuffixCacheEntry::default());
    }
}

#[test]
fn test_suffix_cache_new_large_size() {
    let size = 1000;
    let cache = SuffixCache::new(size);
    assert_eq!(cache.table.len(), size);
    assert_eq!(cache.version, 0);
    for entry in cache.table.iter() {
        assert_eq!(entry, &SuffixCacheEntry::default());
    }
}

