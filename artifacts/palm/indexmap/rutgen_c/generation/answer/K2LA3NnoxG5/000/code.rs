// Answer 0

#[test]
fn test_shift_remove_value_present() {
    struct MockEquivalent;

    impl Hash for MockEquivalent {
        fn hash<H: std::hash::Hasher>(&self, _: &mut H) {}
    }

    impl Equivalent<MockEquivalent> for MockEquivalent {
        fn equivalent(&self, _: &Self) -> bool {
            true
        }
    }

    struct MockIndexMap;

    impl MockIndexMap {
        fn shift_remove<Q>(&mut self, _: &Q) -> Option<MockEquivalent>
        where
            Q: ?Sized + Hash + Equivalent<MockEquivalent>,
        {
            Some(MockEquivalent)
        }
    }

    struct MockIndexSet {
        map: MockIndexMap,
    }

    impl MockIndexSet {
        fn shift_remove<Q>(&mut self, value: &Q) -> bool
        where
            Q: ?Sized + Hash + Equivalent<MockEquivalent>,
        {
            self.map.shift_remove(value).is_some()
        }
    }

    let mut set = MockIndexSet { map: MockIndexMap };
    assert!(set.shift_remove(&MockEquivalent));
}

#[test]
fn test_shift_remove_value_not_present() {
    struct MockEquivalent;

    impl Hash for MockEquivalent {
        fn hash<H: std::hash::Hasher>(&self, _: &mut H) {}
    }

    impl Equivalent<MockEquivalent> for MockEquivalent {
        fn equivalent(&self, _: &Self) -> bool {
            false
        }
    }

    struct MockIndexMap;

    impl MockIndexMap {
        fn shift_remove<Q>(&mut self, _: &Q) -> Option<MockEquivalent>
        where
            Q: ?Sized + Hash + Equivalent<MockEquivalent>,
        {
            None
        }
    }

    struct MockIndexSet {
        map: MockIndexMap,
    }

    impl MockIndexSet {
        fn shift_remove<Q>(&mut self, value: &Q) -> bool
        where
            Q: ?Sized + Hash + Equivalent<MockEquivalent>,
        {
            self.map.shift_remove(value).is_some()
        }
    }

    let mut set = MockIndexSet { map: MockIndexMap };
    assert!(!set.shift_remove(&MockEquivalent));
}

