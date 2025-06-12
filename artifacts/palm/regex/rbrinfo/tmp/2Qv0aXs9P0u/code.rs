fn new(size: usize) -> Self {
        SuffixCache {
            table: vec![SuffixCacheEntry::default(); size],
            version: 0,
        }
    }