// Answer 0

#[test]
fn test_fmt_with_empty_set() {
    use hashbrown::HashSet;
    use std::fmt;

    struct MySet(HashSet<i32>);

    impl fmt::Debug for MySet {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.0.clone()).finish()
        }
    }

    let set = MySet(HashSet::new());
    let result = format!("{:?}", set);
    assert_eq!(result, "[]");
}

#[test]
fn test_fmt_with_single_element_set() {
    use hashbrown::HashSet;
    use std::fmt;

    struct MySet(HashSet<i32>);

    impl fmt::Debug for MySet {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.0.clone()).finish()
        }
    }

    let mut set = MySet(HashSet::new());
    set.0.insert(10);
    let result = format!("{:?}", set);
    assert_eq!(result, "[10]");
}

#[test]
fn test_fmt_with_multiple_elements_set() {
    use hashbrown::HashSet;
    use std::fmt;

    struct MySet(HashSet<i32>);

    impl fmt::Debug for MySet {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_list().entries(self.0.clone()).finish()
        }
    }

    let mut set = MySet(HashSet::new());
    set.0.insert(5);
    set.0.insert(15);
    set.0.insert(25);
    let result = format!("{:?}", set);
    assert_eq!(result, "[5, 15, 25]");
}

