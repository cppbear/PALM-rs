// Answer 0

#[derive(Debug)]
struct SuffixCacheEntry {
    key: SuffixCacheKey,
    pc: InstPtr,
    version: usize,
}

#[derive(Debug)]
struct SuffixCache {
    table: Vec<SuffixCacheEntry>,
    version: usize,
}

impl SuffixCache {
    fn hash(&self, key: &SuffixCacheKey) -> usize {
        // Simple hash function for demonstration purposes
        key.hash_code() % self.table.len()
    }
    
    // The function under test
    fn get(&mut self, key: SuffixCacheKey, pc: InstPtr) -> Option<InstPtr> {
        let h = self.hash(&key);
        let e = &self.table[h];
        if e.key == key && e.version == self.version {
            Some(e.pc)
        } else {
            self.table[h] = SuffixCacheEntry {
                key: key,
                pc: pc,
                version: self.version,
            };
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct SuffixCacheKey {
    value: usize,
}

impl SuffixCacheKey {
    fn hash_code(&self) -> usize {
        self.value
    }
}

type InstPtr = usize;

#[test]
fn test_get_existing_entry() {
    let mut cache = SuffixCache {
        table: vec![SuffixCacheEntry { key: SuffixCacheKey { value: 1 }, pc: 0, version: 0 }; 10],
        version: 1,
    };

    cache.table[0] = SuffixCacheEntry { key: SuffixCacheKey { value: 1 }, pc: 10, version: 1 };
    
    let result = cache.get(SuffixCacheKey { value: 1 }, 15);
    assert_eq!(result, Some(10));
}

#[test]
fn test_get_new_entry() {
    let mut cache = SuffixCache {
        table: vec![SuffixCacheEntry { key: SuffixCacheKey { value: 1 }, pc: 0, version: 0 }; 10],
        version: 1,
    };

    let result = cache.get(SuffixCacheKey { value: 2 }, 15);
    assert_eq!(result, None);
    assert_eq!(cache.table[1].key.value, 2);
    assert_eq!(cache.table[1].pc, 15);
    assert_eq!(cache.table[1].version, 1);
}

#[test]
fn test_get_entry_with_different_version() {
    let mut cache = SuffixCache {
        table: vec![SuffixCacheEntry { key: SuffixCacheKey { value: 1 }, pc: 0, version: 0 }; 10],
        version: 2,
    };

    cache.table[0] = SuffixCacheEntry { key: SuffixCacheKey { value: 1 }, pc: 10, version: 1 };

    let result = cache.get(SuffixCacheKey { value: 1 }, 15);
    assert_eq!(result, None);
    assert_eq!(cache.table[0].key.value, 1);
    assert_eq!(cache.table[0].pc, 15);
    assert_eq!(cache.table[0].version, 2);
}

