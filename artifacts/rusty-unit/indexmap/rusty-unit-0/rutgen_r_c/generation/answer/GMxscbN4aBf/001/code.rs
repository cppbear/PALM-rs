// Answer 0

#[test]
fn test_swap_remove_success() {
    struct DummyHasher;
    struct DummyEquivalent;

    impl Hash for DummyEquivalent {
        fn hash<H: core::hash::Hasher>(&self, _state: &mut H) {}
    }

    impl Equivalent<DummyEquivalent> for DummyEquivalent {
        fn equivalent(&self, _other: &Self) -> bool {
            true
        }
    }

    let mut set: super::IndexSet<DummyEquivalent, DummyHasher> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore {
                // Initialize core structure here
            },
            hash_builder: DummyHasher,
        },
    };

    set.map.insert(DummyEquivalent);
    assert!(set.swap_remove(&DummyEquivalent));
}

#[test]
fn test_swap_remove_not_found() {
    struct DummyHasher;
    struct DummyEquivalent;

    impl Hash for DummyEquivalent {
        fn hash<H: core::hash::Hasher>(&self, _state: &mut H) {}
    }

    impl Equivalent<DummyEquivalent> for DummyEquivalent {
        fn equivalent(&self, _other: &Self) -> bool {
            false
        }
    }

    let mut set: super::IndexSet<DummyEquivalent, DummyHasher> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore {
                // Initialize core structure here
            },
            hash_builder: DummyHasher,
        },
    };

    assert!(!set.swap_remove(&DummyEquivalent));
}

#[test]
fn test_swap_remove_multiple() {
    struct DummyHasher;
    struct DummyEquivalent;

    impl Hash for DummyEquivalent {
        fn hash<H: core::hash::Hasher>(&self, _state: &mut H) {}
    }

    impl Equivalent<DummyEquivalent> for DummyEquivalent {
        fn equivalent(&self, _other: &Self) -> bool {
            true
        }
    }

    let mut set: super::IndexSet<DummyEquivalent, DummyHasher> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore {
                // Initialize core structure here
            },
            hash_builder: DummyHasher,
        },
    };

    set.map.insert(DummyEquivalent);
    assert!(set.swap_remove(&DummyEquivalent));
    assert!(!set.swap_remove(&DummyEquivalent)); // Should return false now
}

