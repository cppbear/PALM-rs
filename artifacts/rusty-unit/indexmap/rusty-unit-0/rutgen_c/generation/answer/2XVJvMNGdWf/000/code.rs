// Answer 0

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TestValue(u32);

struct TestHasher;

impl BuildHasher for TestHasher {
    type Hasher = std::collections::hash_map::RandomState;

    fn build_hasher(&self) -> Self::Hasher {
        Self::Hasher::default()
    }
}

#[test]
fn test_bitxor_empty_sets() {
    let set1: IndexSet<TestValue, TestHasher> = IndexSet::new();
    let set2: IndexSet<TestValue, TestHasher> = IndexSet::new();
    
    let result = &set1 ^ &set2;
    assert!(result.is_empty());
}

#[test]
fn test_bitxor_non_empty_sets() {
    let mut set1: IndexSet<TestValue, TestHasher> = IndexSet::new();
    set1.insert(TestValue(1));
    set1.insert(TestValue(2));

    let mut set2: IndexSet<TestValue, TestHasher> = IndexSet::new();
    set2.insert(TestValue(2));
    set2.insert(TestValue(3));

    let result = &set1 ^ &set2;

    let mut expected: IndexSet<TestValue, TestHasher> = IndexSet::new();
    expected.insert(TestValue(1));
    expected.insert(TestValue(3));

    assert_eq!(result, expected);
}

#[test]
fn test_bitxor_identical_sets() {
    let mut set1: IndexSet<TestValue, TestHasher> = IndexSet::new();
    set1.insert(TestValue(1));
    set1.insert(TestValue(2));

    let result = &set1 ^ &set1;

    assert!(result.is_empty());
}

#[test]
fn test_bitxor_left_empty() {
    let set1: IndexSet<TestValue, TestHasher> = IndexSet::new();
    let mut set2: IndexSet<TestValue, TestHasher> = IndexSet::new();
    set2.insert(TestValue(2));
    
    let result = &set1 ^ &set2;

    let mut expected: IndexSet<TestValue, TestHasher> = IndexSet::new();
    expected.insert(TestValue(2));

    assert_eq!(result, expected);
} 

#[test]
fn test_bitxor_right_empty() {
    let mut set1: IndexSet<TestValue, TestHasher> = IndexSet::new();
    set1.insert(TestValue(1));
    
    let set2: IndexSet<TestValue, TestHasher> = IndexSet::new();

    let result = &set1 ^ &set2;

    let mut expected: IndexSet<TestValue, TestHasher> = IndexSet::new();
    expected.insert(TestValue(1));

    assert_eq!(result, expected);
}

