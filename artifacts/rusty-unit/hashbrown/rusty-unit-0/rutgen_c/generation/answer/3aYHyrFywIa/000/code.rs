// Answer 0

#[test]
fn test_or_default_with_vacant_entry() {
    struct DummyAllocator;
    struct DummyHashBuilder;

    use core::hash::{Hash, Hasher};

    impl BuildHasher for DummyHashBuilder {
        type Hasher = DummyHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DummyHasher
        }
    }

    struct DummyHasher;

    impl Hasher for DummyHasher {
        fn finish(&self) -> u64 {
            0
        }

        fn write(&mut self, _bytes: &[u8]) {}

        fn write_u8(&mut self, _i: u8) {}

        fn write_u16(&mut self, _i: u16) {}

        fn write_u32(&mut self, _i: u32) {}

        fn write_u64(&mut self, _i: u64) {}

        fn write_usize(&mut self, _i: usize) {}

        fn write_i8(&mut self, _i: i8) {}

        fn write_i16(&mut self, _i: i16) {}

        fn write_i32(&mut self, _i: i32) {}

        fn write_i64(&mut self, _i: i64) {}

        fn write_isize(&mut self, _i: isize) {}
    }

    use crate::HashMap;

    let mut map: HashMap<&str, Option<u32, DummyHashBuilder, DummyAllocator>> = HashMap::new();

    let entry = Entry::Vacant(VacantEntry {
        hash: 0,
        key: "poneyland",
        table: &mut map,
    });

    let value_ref = entry.or_default();
    assert_eq!(*value_ref, None);
}

#[test]
fn test_or_default_with_occupied_entry() {
    struct DummyAllocator;
    struct DummyHashBuilder;

    use core::hash::{Hash, Hasher};

    impl BuildHasher for DummyHashBuilder {
        type Hasher = DummyHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DummyHasher
        }
    }

    struct DummyHasher;

    impl Hasher for DummyHasher {
        fn finish(&self) -> u64 {
            0
        }

        fn write(&mut self, _bytes: &[u8]) {}

        fn write_u8(&mut self, _i: u8) {}

        fn write_u16(&mut self, _i: u16) {}

        fn write_u32(&mut self, _i: u32) {}

        fn write_u64(&mut self, _i: u64) {}

        fn write_usize(&mut self, _i: usize) {}

        fn write_i8(&mut self, _i: i8) {}

        fn write_i16(&mut self, _i: i16) {}

        fn write_i32(&mut self, _i: i32) {}

        fn write_i64(&mut self, _i: i64) {}

        fn write_isize(&mut self, _i: isize) {}
    }

    use crate::HashMap;

    let mut map: HashMap<&str, Option<u32, DummyHashBuilder, DummyAllocator>> = HashMap::new();
    map.insert("horseland", Some(3));

    let entry = Entry::Occupied(OccupiedEntry {
        hash: 0,
        elem: Bucket { /* Populate with necessary fields */ },
        table: &mut map,
    });

    let value_ref = entry.or_default();
    assert_eq!(*value_ref, Some(3));
}

