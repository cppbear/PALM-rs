// Answer 0

#[test]
fn test_equivalent_function() {
    struct Key;
    struct EquivalentImpl;

    impl Equivalent<Key> for EquivalentImpl {
        fn equivalent(&self, _: &Key) -> bool {
            true
        }
    }

    let equivalent_impl = EquivalentImpl;
    let closure = equivalent(&equivalent_impl);

    let key = Key;
    assert!(closure(&key));
}

#[test]
fn test_equivalent_function_false() {
    struct Key;
    struct EquivalentImplFalse;

    impl Equivalent<Key> for EquivalentImplFalse {
        fn equivalent(&self, _: &Key) -> bool {
            false
        }
    }

    let equivalent_impl_false = EquivalentImplFalse;
    let closure = equivalent(&equivalent_impl_false);

    let key = Key;
    assert!(!closure(&key));
}

