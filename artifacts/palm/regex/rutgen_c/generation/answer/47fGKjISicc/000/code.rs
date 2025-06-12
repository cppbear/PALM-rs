// Answer 0

#[test]
fn test_get_existing_entry() {
    let mut cache = SuffixCache::new(10);
    let key = SuffixCacheKey { from_inst: 1, start: 1, end: 2 };
    let pc: InstPtr = 100;

    // Set an entry in the cache
    cache.table[cache.hash(&key)] = SuffixCacheEntry {
        key: key,
        pc: pc,
        version: cache.version,
    };

    // Fetch the entry
    let result = cache.get(key, 200);

    assert_eq!(result, Some(pc));
}

#[test]
fn test_get_non_existing_entry() {
    let mut cache = SuffixCache::new(10);
    let key = SuffixCacheKey { from_inst: 1, start: 1, end: 2 };
    let pc: InstPtr = 100;

    // Attempt to fetch a non-existing entry
    let result = cache.get(key, pc);

    assert_eq!(result, None);
}

#[test]
fn test_get_with_new_entry() {
    let mut cache = SuffixCache::new(10);
    let key = SuffixCacheKey { from_inst: 1, start: 1, end: 2 };
    let pc: InstPtr = 100;

    // Fetch a non-existing entry, should insert and return None
    let result = cache.get(key, pc);

    assert_eq!(result, None);

    // Verify that the entry was inserted
    let inserted_entry = cache.table[cache.hash(&key)];
    assert_eq!(inserted_entry.key, key);
    assert_eq!(inserted_entry.pc, pc);
    assert_eq!(inserted_entry.version, cache.version);
}

#[test]
fn test_get_different_key() {
    let mut cache = SuffixCache::new(10);
    let key1 = SuffixCacheKey { from_inst: 1, start: 1, end: 2 };
    let key2 = SuffixCacheKey { from_inst: 2, start: 2, end: 3 };
    let pc1: InstPtr = 100;
    let pc2: InstPtr = 200;

    // Insert the first key
    cache.get(key1, pc1);

    // Fetch with a different key
    let result = cache.get(key2, pc2);

    assert_eq!(result, None);

    // Verify that the first key remains unchanged
    let entry = cache.table[cache.hash(&key1)];
    assert_eq!(entry.pc, pc1);
}

