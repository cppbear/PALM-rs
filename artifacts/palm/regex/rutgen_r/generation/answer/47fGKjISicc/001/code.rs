// Answer 0

#[derive(Debug, Clone)]
struct SuffixCacheKey {
    id: usize,
}

#[derive(Debug)]
struct SuffixCacheEntry {
    key: SuffixCacheKey,
    pc: InstPtr,
    version: usize,
}

struct InstPtr(usize);

struct SuffixCache {
    table: Vec<SuffixCacheEntry>,
    version: usize,
}

impl SuffixCache {
    fn hash(&self, key: &SuffixCacheKey) -> usize {
        key.id % self.table.len()
    }
    
    fn get(&mut self, key: SuffixCacheKey, pc: InstPtr) -> Option<InstPtr> {
        let h = self.hash(&key);
        let e = &mut self.table[h];
        if e.key == key && e.version == self.version {
            Some(e.pc)
        } else {
            *e = SuffixCacheEntry {
                key: key,
                pc: pc,
                version: self.version,
            };
            None
        }
    }
}

#[test]
fn test_get_existing_entry() {
    let key = SuffixCacheKey { id: 0 };
    let pc = InstPtr(42);
    let mut cache = SuffixCache {
        table: vec![SuffixCacheEntry {
            key: key.clone(),
            pc: InstPtr(100),
            version: 1,
        }; 10],
        version: 1,
    };
    
    // This should return Some(100) since the key and version match
    assert_eq!(cache.get(key.clone(), pc), Some(InstPtr(100)));
}

#[test]
fn test_get_non_existing_entry() {
    let key1 = SuffixCacheKey { id: 0 };
    let key2 = SuffixCacheKey { id: 1 };
    let pc = InstPtr(42);
    let mut cache = SuffixCache {
        table: vec![SuffixCacheEntry {
            key: key1.clone(),
            pc: InstPtr(100),
            version: 1,
        }; 10],
        version: 1,
    };
    
    // This should fill the entry with the new PC and return None
    assert_eq!(cache.get(key2.clone(), pc), None);
    // Verify that the new entry was added
    assert_eq!(cache.get(key2.clone(), pc), None);
    assert_eq!(cache.table[0].pc, pc);
}

#[test]
fn test_get_version_mismatch() {
    let key = SuffixCacheKey { id: 0 };
    let pc = InstPtr(42);
    let mut cache = SuffixCache {
        table: vec![SuffixCacheEntry {
            key: key.clone(),
            pc: InstPtr(100),
            version: 2,
        }; 10],
        version: 1,
    };
    
    // Version does not match, so it should store the new entry and return None
    assert_eq!(cache.get(key.clone(), pc), None);
    assert_eq!(cache.table[0].pc, pc);
}

#[test]
fn test_get_key_mismatch() {
    let key1 = SuffixCacheKey { id: 0 };
    let key2 = SuffixCacheKey { id: 1 };
    let pc1 = InstPtr(100);
    let pc2 = InstPtr(200);
    let mut cache = SuffixCache {
        table: vec![SuffixCacheEntry {
            key: key1.clone(),
            pc: pc1,
            version: 1,
        }; 10],
        version: 1,
    };
    
    // It should replace the entry and return None for key2
    assert_eq!(cache.get(key2.clone(), pc2), None);
    // Verify that key2's entry is present now
    assert_eq!(cache.table[0].pc, pc2);
}

