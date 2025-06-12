// Answer 0

#[test]
fn test_class_unicode_iter_empty() {
    struct TestClass {
        set: Vec<u32>,
    }

    impl TestClass {
        pub fn iter(&self) -> ClassUnicodeIter {
            ClassUnicodeIter(self.set.iter())
        }
    }

    struct ClassUnicodeIter<'a>(std::slice::Iter<'a, u32>);

    let class = TestClass { set: vec![] };
    let mut iter = class.iter();

    assert!(iter.0.next().is_none());
}

#[test]
fn test_class_unicode_iter_single_element() {
    struct TestClass {
        set: Vec<u32>,
    }

    impl TestClass {
        pub fn iter(&self) -> ClassUnicodeIter {
            ClassUnicodeIter(self.set.iter())
        }
    }

    struct ClassUnicodeIter<'a>(std::slice::Iter<'a, u32>);

    let class = TestClass { set: vec![100] };
    let mut iter = class.iter();

    assert_eq!(iter.0.next(), Some(&100));
    assert!(iter.0.next().is_none());
}

#[test]
fn test_class_unicode_iter_multiple_elements() {
    struct TestClass {
        set: Vec<u32>,
    }

    impl TestClass {
        pub fn iter(&self) -> ClassUnicodeIter {
            ClassUnicodeIter(self.set.iter())
        }
    }

    struct ClassUnicodeIter<'a>(std::slice::Iter<'a, u32>);

    let class = TestClass { set: vec![100, 200, 300] };
    let mut iter = class.iter();

    let results: Vec<_> = iter.0.collect();
    assert_eq!(results, vec![&100, &200, &300]);
}

#[test]
fn test_class_unicode_iter_large_set() {
    struct TestClass {
        set: Vec<u32>,
    }

    impl TestClass {
        pub fn iter(&self) -> ClassUnicodeIter {
            ClassUnicodeIter(self.set.iter())
        }
    }

    struct ClassUnicodeIter<'a>(std::slice::Iter<'a, u32>);

    let class = TestClass { set: (1..=1000).collect() };
    let mut iter = class.iter();

    let results: Vec<_> = iter.0.collect();
    assert_eq!(results.len(), 1000);
    assert_eq!(results.first(), Some(&1));
    assert_eq!(results.last(), Some(&1000));
}

#[test]
fn test_class_unicode_iter_duplicate_elements() {
    struct TestClass {
        set: Vec<u32>,
    }

    impl TestClass {
        pub fn iter(&self) -> ClassUnicodeIter {
            ClassUnicodeIter(self.set.iter())
        }
    }

    struct ClassUnicodeIter<'a>(std::slice::Iter<'a, u32>);

    let class = TestClass { set: vec![100, 200, 200, 300] };
    let mut iter = class.iter();

    let results: Vec<_> = iter.0.collect();
    assert_eq!(results, vec![&100, &200, &200, &300]);
}

