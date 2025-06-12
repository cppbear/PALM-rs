// Answer 0

#[derive(Debug)]
struct TestKey {
    value: i32,
}

impl PartialEq for TestKey {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Hash for TestKey {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

struct TestEquivalent;

impl Equivalent<TestKey> for TestEquivalent {
    fn equivalent(&self, a: &TestKey) -> bool {
        a.value % 2 == 0 // Example equivalence check: even numbers are equivalent
    }
}

#[test]
fn test_equivalent_key_with_equivalent_closure() {
    let k = TestEquivalent;
    let equivalent_function = equivalent_key(&k);
    
    let pair1 = (TestKey { value: 2 }, "First");
    let pair2 = (TestKey { value: 4 }, "Second");
    let pair3 = (TestKey { value: 3 }, "Third");

    assert!(equivalent_function(&pair1));
    assert!(equivalent_function(&pair2));
    assert!(!equivalent_function(&pair3));
}

#[test]
fn test_equivalent_key_with_different_values() {
    let k = TestEquivalent;
    let equivalent_function = equivalent_key(&k);
    
    let pair1 = (TestKey { value: 2 }, "First");
    let pair2 = (TestKey { value: 6 }, "Second");

    assert!(equivalent_function(&pair1));
    assert!(equivalent_function(&pair2));
}

#[test]
fn test_equivalent_key_with_same_key_different_pairs() {
    let k = TestEquivalent;
    let equivalent_function = equivalent_key(&k);
    
    let pair1 = (TestKey { value: 2 }, "First");
    let pair2 = (TestKey { value: 2 }, "Second");

    assert!(equivalent_function(&pair1));
    assert!(equivalent_function(&pair2));
}

