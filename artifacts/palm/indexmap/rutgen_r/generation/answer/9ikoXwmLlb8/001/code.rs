// Answer 0

#[derive(Debug)]
struct Item<K, V> {
    key: K,
    value: V,
}

impl<K, V> Item<K, V> {
    fn new(key: K, value: V) -> Self {
        Item { key, value }
    }

    fn refs(&self) -> (&K, &V) {
        (&self.key, &self.value)
    }
}

#[test]
fn test_refs_valid() {
    let item = Item::new("test_key", 42);
    let (key_ref, value_ref) = item.refs();
    
    assert_eq!(*key_ref, "test_key");
    assert_eq!(*value_ref, 42);
}

#[test]
fn test_refs_with_different_types() {
    let item = Item::new(1.5, "value");
    let (key_ref, value_ref) = item.refs();
    
    assert_eq!(*key_ref, 1.5);
    assert_eq!(*value_ref, "value");
}

#[test]
#[should_panic]
fn test_refs_panic_on_mutation() {
    let mut item = Item::new("key", "value");
    {
        let (key_ref, value_ref) = item.refs();
        // Trying to mutate would panic if we had mutable references that are kept alive
        // This is not directly coded but illustrates the concept of maintaining references safely.
        // In this case, we are not modifying anything, but this serves as a guard against 
        // unintentional changes while the reference holds.
        assert_eq!(*key_ref, "key");
        assert_eq!(*value_ref, "value");
    }
    // Simulating a panic scenario by going out of scope (not the case here directly)
    // Inserting an element here would typically be out of context and should lead to unsafe behavior,
    // but also shows the potential for misunderstanding if references are retained incorrectly.
}

