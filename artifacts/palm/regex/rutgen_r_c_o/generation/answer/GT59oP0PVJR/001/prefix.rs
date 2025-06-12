// Answer 0

#[test]
fn test_clear_no_items() {
    let mut suffix_cache = SuffixCache::new(0);
    suffix_cache.clear();
}

#[test]
fn test_clear_small_size() {
    let mut suffix_cache = SuffixCache::new(10);
    suffix_cache.clear();
}

#[test]
fn test_clear_medium_size() {
    let mut suffix_cache = SuffixCache::new(500);
    suffix_cache.clear();
}

#[test]
fn test_clear_large_size() {
    let mut suffix_cache = SuffixCache::new(1000);
    suffix_cache.clear();
}

#[test]
fn test_clear_twice() {
    let mut suffix_cache = SuffixCache::new(100);
    suffix_cache.clear();
    suffix_cache.clear();
}

