// Answer 0

#[derive(Default)]
struct TestHasher {
    value: usize,
}

impl TestHasher {
    fn new(value: usize) -> Self {
        TestHasher { value }
    }
}

impl std::hash::Hasher for TestHasher {
    fn finish(&self) -> u64 {
        self.value as u64
    }

    fn write(&mut self, _: &[u8]) {
        // No-op for testing
    }

    fn write_u8(&mut self, _: u8) {
        // No-op for testing
    }
}

struct TestMap<S: std::hash::BuildHasher> {
    hasher: S,
}

impl<S: std::hash::BuildHasher> TestMap<S> {
    fn new(hasher: S) -> Self {
        TestMap { hasher }
    }

    fn hasher(&self) -> &S {
        &self.hasher
    }
}

struct MySet<S: std::hash::BuildHasher> {
    map: TestMap<S>,
}

impl<S: std::hash::BuildHasher> MySet<S> {
    fn new(map: TestMap<S>) -> Self {
        MySet { map }
    }

    pub fn hasher(&self) -> &S {
        self.map.hasher()
    }
}

#[test]
fn test_hasher_return_value() {
    let hasher = TestHasher::new(42);
    let test_map = TestMap::new(hasher);
    let my_set = MySet::new(test_map);

    assert_eq!(my_set.hasher().finish(), 42);
}

#[test]
#[should_panic]
fn test_hasher_panic() {
    // Setup test with an uninitialized hasher for panic conditions.
    struct EmptyHasher;
    impl std::hash::BuildHasher for EmptyHasher {
        type Hasher = EmptyHasher;
        fn build_hasher(&self) -> Self::Hasher {
            EmptyHasher
        }
    }

    impl std::hash::Hasher for EmptyHasher {
        fn finish(&self) -> u64 {
            panic!("Panic from EmptyHasher");
        }

        fn write(&mut self, _: &[u8]) {}
        fn write_u8(&mut self, _: u8) {}
    }

    let empty_hasher = EmptyHasher;
    let test_map = TestMap::new(empty_hasher);
    let my_set = MySet::new(test_map);

    my_set.hasher().finish(); // This should trigger a panic.
}

