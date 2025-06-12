// Answer 0

#[derive(Debug)]
struct TestHasher {
    value: u64,
}

impl TestHasher {
    fn new(value: u64) -> Self {
        TestHasher { value }
    }
}

#[derive(Debug)]
struct TestMap<S> {
    hash_builder: S,
}

impl<S> TestMap<S> {
    fn new(hash_builder: S) -> Self {
        TestMap { hash_builder }
    }
    
    pub fn hasher(&self) -> &S {
        &self.hash_builder
    }
}

#[test]
fn test_hasher_valid_reference() {
    let hasher = TestHasher::new(42);
    let map = TestMap::new(hasher);
    assert_eq!(map.hasher().value, 42);
}

#[test]
fn test_hasher_return_type() {
    let hasher = TestHasher::new(100);
    let map = TestMap::new(hasher);
    let returned_hasher = map.hasher();
    assert!(std::any::TypeId::of::<&TestHasher>() == std::any::TypeId::of_val(returned_hasher));
}

