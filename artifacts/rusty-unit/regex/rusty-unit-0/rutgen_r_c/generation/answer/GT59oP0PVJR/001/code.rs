// Answer 0

#[test]
fn test_clear_increments_version() {
    let mut suffix_cache = SuffixCache::new(10);
    let initial_version = suffix_cache.version;
    
    suffix_cache.clear();
    
    assert_eq!(suffix_cache.version, initial_version + 1);
}

#[test]
fn test_clear_multiple_calls() {
    let mut suffix_cache = SuffixCache::new(5);
    let initial_version = suffix_cache.version;
    
    suffix_cache.clear();
    suffix_cache.clear();
    
    assert_eq!(suffix_cache.version, initial_version + 2);
}

#[test]
fn test_clear_on_empty_cache() {
    let mut suffix_cache = SuffixCache::new(0); // size 0 cache
    let initial_version = suffix_cache.version;
    
    suffix_cache.clear();
    
    assert_eq!(suffix_cache.version, initial_version + 1);
}

