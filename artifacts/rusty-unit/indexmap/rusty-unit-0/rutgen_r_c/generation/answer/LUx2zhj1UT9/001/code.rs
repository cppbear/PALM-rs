// Answer 0

#[test]
fn test_union_fmt_debug() {
    use std::fmt::Formatter;
    use std::collections::HashSet;
    use std::hash::{BuildHasherDefault, Hasher};
    use core::fmt;

    struct TestHasher {
        value: u64,
    }

    impl Hasher for TestHasher {
        fn write(&mut self, bytes: &[u8]) {
            self.value = self.value.wrapping_add(bytes.len() as u64);
        }

        fn finish(&self) -> u64 {
            self.value
        }
    }

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct TestElement(u32);

    struct TestIndexSet {
        elements: HashSet<TestElement, BuildHasherDefault<TestHasher>>,
    }

    impl TestIndexSet {
        fn new() -> Self {
            Self {
                elements: HashSet::with_hasher(BuildHasherDefault::default()),
            }
        }

        fn insert(&mut self, elem: TestElement) {
            self.elements.insert(elem);
        }
    }

    let mut index_set = TestIndexSet::new();
    index_set.insert(TestElement(1));
    index_set.insert(TestElement(2));

    let union = Union {
        iter: Chain::new(Iter { iter: index_set.elements.iter() }, Difference { iter: Iter { iter: [] }, other: &index_set }),
    };

    let mut output = String::new();
    let mut buffer = Formatter::new(&mut output);
    union.fmt(&mut buffer).expect("Formatting failed");

    assert_eq!(output, "[1, 2]");
} 

#[test]
fn test_union_fmt_empty_union() {
    use std::fmt::Formatter;
    use core::fmt;

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct TestElement(u32);

    struct TestIndexSet {
        elements: HashSet<TestElement>,
    }

    impl TestIndexSet {
        fn new() -> Self {
            Self {
                elements: HashSet::new(),
            }
        }
    }

    let index_set = TestIndexSet::new();
    let union = Union {
        iter: Chain::new(Iter { iter: [] }, Difference { iter: Iter { iter: [] }, other: &index_set }),
    };

    let mut output = String::new();
    let mut buffer = Formatter::new(&mut output);
    union.fmt(&mut buffer).expect("Formatting failed");

    assert_eq!(output, "[]");
} 

#[should_panic]
#[test]
fn test_union_fmt_invalid_state() {
    use std::fmt::Formatter;
    use std::collections::HashSet;

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct TestElement(u32);

    struct TestIndexSet {
        elements: HashSet<TestElement>,
    }

    impl TestIndexSet {
        fn new() -> Self {
            Self {
                elements: HashSet::new(),
            }
        }
    }

    let index_set = TestIndexSet::new();
    let union = Union { iter: Chain::new(Iter { iter: [] }, Difference { iter: Iter { iter: [] }, other: &index_set }) };

    let mut buffer = Formatter::new(&mut String::new());
    // Assume union is in an invalid state for testing panic
    let _ = union.fmt(&mut buffer);
} 

