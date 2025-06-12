// Answer 0

#[derive(Debug)]
struct SuffixCacheEntry {
    key: String,
    pc: usize,
    version: usize,
}

struct SuffixCache {
    table: Vec<SuffixCacheEntry>,
    version: usize,
}

impl SuffixCache {
    fn hash(&self, key: &str) -> usize {
        key.len() % self.table.len() // Simple hash for demonstration
    }

    fn get(&mut self, key: String, pc: usize) -> Option<usize> {
        let h = self.hash(&key);
        let e = &self.table[h];
        if e.key == key && e.version == self.version {
            Some(e.pc)
        } else {
            self.table[h] = SuffixCacheEntry {
                key,
                pc,
                version: self.version,
            };
            None
        }
    }
}

#[test]
fn test_get_none_case_due_to_different_key() {
    let mut cache = SuffixCache {
        table: vec![SuffixCacheEntry {
            key: "existing_key".to_string(),
            pc: 1,
            version: 0,
        }; 10], // Initialize with 10 entries
        version: 1,
    };

    let result = cache.get("new_key".to_string(), 2);
    
    assert_eq!(result, None);
}

#[test]
fn test_get_none_case_with_empty_table() {
    let mut cache = SuffixCache {
        table: vec![], // Initialize with an empty table
        version: 1,
    };

    let result = cache.get("some_key".to_string(), 3);
    
    assert_eq!(result, None);
}

