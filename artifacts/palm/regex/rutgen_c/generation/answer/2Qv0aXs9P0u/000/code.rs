// Answer 0

#[test]
fn test_suffix_cache_new() {
    let size = 10;
    let cache = SuffixCache::new(size);
    assert_eq!(cache.table.len(), size);
    assert_eq!(cache.version, 0);

    let empty_entry = SuffixCacheEntry::default();
    for entry in cache.table.iter() {
        assert_eq!(*entry, empty_entry);
    }
}

#[test]
fn test_suffix_cache_new_zero_size() {
    let size = 0;
    let cache = SuffixCache::new(size);
    assert_eq!(cache.table.len(), size);
    assert_eq!(cache.version, 0);
}

