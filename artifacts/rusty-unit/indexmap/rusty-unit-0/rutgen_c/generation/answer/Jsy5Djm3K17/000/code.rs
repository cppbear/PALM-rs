// Answer 0

#[test]
fn test_symmetric_difference_debug_fmt() {
    use std::collections::HashSet;
    use std::fmt::Debug;
    use core::hash::BuildHasherDefault;

    struct TestStruct(i32);
    
    // Ensure TestStruct implements Eq and Hash
    impl PartialEq for TestStruct {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
    
    impl Eq for TestStruct {}
    
    impl Hash for TestStruct {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    let set1: IndexSet<TestStruct, BuildHasherDefault<core::hash::Hasher>> = 
        IndexSet::from_iter(vec![TestStruct(1), TestStruct(2)]);
    let set2: IndexSet<TestStruct, BuildHasherDefault<core::hash::Hasher>> = 
        IndexSet::from_iter(vec![TestStruct(2), TestStruct(3)]);

    let symmetric_difference = SymmetricDifference {
        iter: set1.difference(&set2).chain(set2.difference(&set1)),
    };

    // This should compile and run, testing the debug output
    let debug_output = format!("{:?}", symmetric_difference);
    assert!(debug_output.contains("TestStruct(1)") || debug_output.contains("TestStruct(3)"));
}

