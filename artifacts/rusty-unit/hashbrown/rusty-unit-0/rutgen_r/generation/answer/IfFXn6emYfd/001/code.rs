// Answer 0

#[test]
fn test_get_or_insert_with_new_value() {
    use hashbrown::HashSet;
    use std::hash::{Hash, Hasher};

    struct Dummy {
        id: i32,
    }

    impl PartialEq for Dummy {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    struct DummyHasher;

    impl Hasher for DummyHasher {
        fn finish(&self) -> u64 {
            0
        }

        fn write(&mut self, _: &[u8]) {}
    }

    impl Hash for Dummy {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.id.hash(state);
        }
    }

    struct DummyEquivalent;

    impl<Q: Hash> hashbrown::Equivalent<Dummy> for DummyEquivalent {
        fn equivalent(&self, _: &Dummy) -> bool {
            true
        }
    }

    let mut set: HashSet<Dummy> = HashSet::new();
    let dummy_value = Dummy { id: 1 };
    let value_ref = set.get_or_insert_with(&dummy_value, |_: &Dummy| Dummy { id: 1 });

    assert_eq!(value_ref.id, dummy_value.id);
}

#[should_panic]
#[test]
fn test_get_or_insert_with_panic() {
    use hashbrown::HashSet;

    struct PanicDummy {
        id: i32,
    }

    impl PartialEq for PanicDummy {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    struct DummyEquivalent;

    impl<Q: Hash> hashbrown::Equivalent<PanicDummy> for DummyEquivalent {
        fn equivalent(&self, _: &PanicDummy) -> bool {
            false // Intentionally make this return false to trigger panic
        }
    }

    let mut set = HashSet::new();
    set.get_or_insert_with(&PanicDummy { id: 1 }, |_| PanicDummy { id: 2 });
}

