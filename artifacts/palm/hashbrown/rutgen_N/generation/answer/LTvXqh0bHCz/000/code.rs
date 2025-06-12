// Answer 0

#[test]
fn test_fmt() {
    use std::fmt;
    use hashbrown::HashSet; // Assuming HashSet is the context for self
    
    struct TestSet {
        inner: HashSet<i32>,
    }
    
    impl fmt::Debug for TestSet {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.inner.clone()).finish()
        }
    }

    let mut set = TestSet {
        inner: HashSet::new(),
    };
    
    set.inner.insert(1);
    set.inner.insert(2);
    set.inner.insert(3);

    let result = format!("{:?}", set);
    assert_eq!(result, "[1, 2, 3]");
}

#[test]
fn test_fmt_empty_set() {
    use std::fmt;
    use hashbrown::HashSet; // Assuming HashSet is the context for self
    
    struct TestSet {
        inner: HashSet<i32>,
    }

    impl fmt::Debug for TestSet {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.inner.clone()).finish()
        }
    }

    let set = TestSet {
        inner: HashSet::new(),
    };

    let result = format!("{:?}", set);
    assert_eq!(result, "[]");
}

