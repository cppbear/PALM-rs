// Answer 0

#[test]
fn test_suffix_cache_get_entry_not_found() {
    let mut cache = SuffixCache::new(10);
    let key = SuffixCacheKey {
        from_inst: 1,
        start: b'a',
        end: b'z',
    };
    let pc: InstPtr = 42; // Arbitrary InstPtr value
    
    // Ensure the initial state returns None, as the entry does not exist in the cache
    let result = cache.get(key, pc);
    assert_eq!(result, None);
}

#[test]
fn test_suffix_cache_get_entry_with_existing_key_different_version() {
    let mut cache = SuffixCache::new(10);
    let key = SuffixCacheKey {
        from_inst: 1,
        start: b'a',
        end: b'z',
    };
    let pc: InstPtr = 42; // Arbitrary InstPtr value

    // Initially populate the cache with a different key/version
    let different_key = SuffixCacheKey {
        from_inst: 2,
        start: b'x',
        end: b'y',
    };
    let different_pc: InstPtr = 99;
    cache.table[cache.hash(&different_key)] = SuffixCacheEntry {
        key: different_key,
        pc: different_pc,
        version: cache.version,
    };

    // Now querying with the original key should return None
    let result = cache.get(key, pc);
    assert_eq!(result, None);
}

#[test]
fn test_suffix_cache_get_entry_overwrite_and_return_none() {
    let mut cache = SuffixCache::new(10);
    let key = SuffixCacheKey {
        from_inst: 1,
        start: b'a',
        end: b'z',
    };
    let pc: InstPtr = 42; // Arbitrary InstPtr value

    // First access should store the entry as it doesn't exist
    let result_first = cache.get(key, pc);
    assert_eq!(result_first, None);

    // Now check that accessing the same key again updates the entry without returning a value
    let result_second = cache.get(key, pc + 1);
    assert_eq!(result_second, None);
}

