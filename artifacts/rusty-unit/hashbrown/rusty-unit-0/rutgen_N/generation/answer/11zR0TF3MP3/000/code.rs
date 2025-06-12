// Answer 0

#[derive(Debug)]
struct KeyWrapper<'a>(&'a str);

trait Equivalent<K> {
    fn equivalent(&self, other: &K) -> bool;
}

impl Equivalent<KeyWrapper<'_>> for KeyWrapper<'_> {
    fn equivalent(&self, other: &KeyWrapper) -> bool {
        self.0 == other.0
    }
}

#[test]
fn test_equivalent_same_key() {
    let key1 = KeyWrapper("test");
    let key2 = KeyWrapper("test");
    let eq_fn = equivalent(&key1);
    assert!(eq_fn(&key2));
}

#[test]
fn test_equivalent_different_key() {
    let key1 = KeyWrapper("test");
    let key2 = KeyWrapper("different");
    let eq_fn = equivalent(&key1);
    assert!(!eq_fn(&key2));
}

#[test]
fn test_equivalent_empty_key() {
    let key1 = KeyWrapper("");
    let key2 = KeyWrapper("");
    let eq_fn = equivalent(&key1);
    assert!(eq_fn(&key2));
}

#[test]
fn test_equivalent_empty_and_non_empty_key() {
    let key1 = KeyWrapper("");
    let key2 = KeyWrapper("non-empty");
    let eq_fn = equivalent(&key1);
    assert!(!eq_fn(&key2));
}

