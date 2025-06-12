// Answer 0

#[derive(Debug)]
struct KeyedItem<K> {
    key: K,
}

impl<K> KeyedItem<K> {
    fn new(key: K) -> Self {
        KeyedItem { key }
    }

    fn key_ref(&self) -> &K {
        &self.key
    }
}

#[test]
fn test_key_ref() {
    let item = KeyedItem::new("example_key");
    assert_eq!(item.key_ref(), &"example_key");
}

#[test]
fn test_key_ref_empty_string() {
    let item = KeyedItem::new("");
    assert_eq!(item.key_ref(), &"");
}

#[test]
fn test_key_ref_numeric() {
    let item = KeyedItem::new(42);
    assert_eq!(item.key_ref(), &42);
}

#[test]
fn test_key_ref_negative_number() {
    let item = KeyedItem::new(-1);
    assert_eq!(item.key_ref(), &-1);
}

