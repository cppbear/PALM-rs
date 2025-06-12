fn get(&mut self, key: SuffixCacheKey, pc: InstPtr) -> Option<InstPtr> {
        let h = self.hash(&key);
        let e = self.table[h];
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