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

    fn muts(&mut self) -> (&mut K, &mut V) {
        (&mut self.key, &mut self.value)
    }
}

#[test]
fn test_muts() {
    let mut item = Item::new(42, "value");
    let (key, value) = item.muts();
    
    *key = 100;
    *value = "updated value";
    
    assert_eq!(*key, 100);
    assert_eq!(*value, "updated value");
}

#[test]
fn test_muts_boundary_conditions() {
    let mut item = Item::new(0, "");
    let (key, value) = item.muts();
    
    *key = 1;
    *value = "boundary test";
    
    assert_eq!(*key, 1);
    assert_eq!(*value, "boundary test");
}

