// Answer 0

#[test]
fn test_new_size_0() {
    let size: usize = 0;
    let cache = SuffixCache::new(size);
}

#[test]
fn test_new_size_1() {
    let size: usize = 1;
    let cache = SuffixCache::new(size);
}

#[test]
fn test_new_size_1000() {
    let size: usize = 1000;
    let cache = SuffixCache::new(size);
}

#[test]
fn test_new_size_10000() {
    let size: usize = 10000;
    let cache = SuffixCache::new(size);
}

#[test]
fn test_new_size_max() {
    let size: usize = usize::MAX;
    let cache = SuffixCache::new(size);
}

