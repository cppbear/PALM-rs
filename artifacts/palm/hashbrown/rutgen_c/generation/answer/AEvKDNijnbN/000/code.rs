// Answer 0

#[test]
fn test_build_hashes_inner() {
    use std::hash::Hasher;
    use std::collections::hash_map::DefaultHasher;

    struct SimpleHashBuilder;
    impl BuildHasher for SimpleHashBuilder {
        type Hash = DefaultHasher;
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    struct TestKey {
        value: u32,
    }

    impl Hash for TestKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            state.write_u32(self.value);
        }
    }

    impl Equivalent<TestKey> for TestKey {
        fn equivalent(&self, other: &Self) -> bool {
            self.value == other.value
        }
    }

    let hash_builder = SimpleHashBuilder;
    let keys = [
        &TestKey { value: 1 } as &dyn Hash,
        &TestKey { value: 2 } as &dyn Hash,
        &TestKey { value: 3 } as &dyn Hash,
    ];

    let mut hashmap: HashMap<TestKey, u32, SimpleHashBuilder> = HashMap {
        hash_builder,
        table: RawTable {
            table: RawTableInner { /* internal details */ },
            alloc: Global,
            marker: PhantomData,
        },
    };

    let hashes = hashmap.build_hashes_inner(&keys);

    assert_eq!(hashes.len(), 3);
}

