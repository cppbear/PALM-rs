// Answer 0

#[test]
fn test_get_function_inserts_new_entry() {
    // Initialize SuffixCache with size of 5
    let mut cache = SuffixCache::new(5);
    let key = SuffixCacheKey { from_inst: 1, start: 2, end: 3 };
    let pc: InstPtr = 123; // Sample InstPtr value

    // First call should insert the entry and return None since it's empty
    assert_eq!(cache.get(key, pc), None);

    // Retrieve entry after insertion to verify it's stored correctly
    let h = cache.hash(&key);
    let e = cache.table[h];
    
    // Check that the entry has been inserted
    assert_eq!(e.key, key);
    assert_eq!(e.pc, pc);
    assert_eq!(e.version, cache.version);
}

#[test]
fn test_get_function_updates_existing_entry() {
    // Initialize SuffixCache with size of 5
    let mut cache = SuffixCache::new(5);
    let key = SuffixCacheKey { from_inst: 1, start: 2, end: 3 };
    let initial_pc: InstPtr = 123; // Sample InstPtr value

    // First call should insert the entry and return None since it's empty
    assert_eq!(cache.get(key, initial_pc), None);

    // Now, we should check the cache with the same key but a different pc
    let new_pc: InstPtr = 456; // New InstPtr value
    
    // Call get with the same key, expect None and it should update the entry
    assert_eq!(cache.get(key, new_pc), None);
    
    // Retrieve the entry to check if it was updated
    let h = cache.hash(&key);
    let e = cache.table[h];
    
    // Check that the new entry was stored
    assert_eq!(e.key, key);
    assert_eq!(e.pc, new_pc);
    assert_eq!(e.version, cache.version);
}

#[test]
fn test_get_function_does_not_match_on_different_version() {
    // Initialize SuffixCache with size of 5
    let mut cache = SuffixCache::new(5);
    let key = SuffixCacheKey { from_inst: 1, start: 2, end: 3 };
    let pc: InstPtr = 123; // Sample InstPtr value

    // First call should insert the entry and return None since it's empty
    assert_eq!(cache.get(key, pc), None);

    // Simulate a version increment
    cache.version += 1;

    // Call get on the same key with the same pc, expect None due to version mismatch
    assert_eq!(cache.get(key, pc), None);
   
    // Verify that the entry is still there but with incremented version
    let h = cache.hash(&key);
    let e = cache.table[h];
    assert_eq!(e.key, key);
    assert_eq!(e.version, cache.version);
}

