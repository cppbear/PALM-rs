// Answer 0

#[test]
fn test_fmt_with_empty_iter() {
    struct Bucket {
        value: i32,
    }

    struct MyIter {
        iter: Vec<Bucket>,
    }

    impl MyIter {
        fn new(iter: Vec<Bucket>) -> Self {
            MyIter { iter }
        }

        fn iter(&self) -> &[Bucket] {
            &self.iter
        }
    }

    impl std::fmt::Debug for MyIter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let iter = self.iter();
            f.debug_list().entries(iter.iter().map(|b| &b.value)).finish()
        }
    }

    let my_iter = MyIter::new(vec![]);
    let result = format!("{:?}", my_iter);
    assert_eq!(result, "[]");
}

#[test]
fn test_fmt_with_single_element() {
    struct Bucket {
        value: i32,
    }

    struct MyIter {
        iter: Vec<Bucket>,
    }

    impl MyIter {
        fn new(iter: Vec<Bucket>) -> Self {
            MyIter { iter }
        }

        fn iter(&self) -> &[Bucket] {
            &self.iter
        }
    }

    impl std::fmt::Debug for MyIter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let iter = self.iter();
            f.debug_list().entries(iter.iter().map(|b| &b.value)).finish()
        }
    }

    let my_iter = MyIter::new(vec![Bucket { value: 42 }]);
    let result = format!("{:?}", my_iter);
    assert_eq!(result, "[42]");
}

#[test]
fn test_fmt_with_multiple_elements() {
    struct Bucket {
        value: i32,
    }

    struct MyIter {
        iter: Vec<Bucket>,
    }

    impl MyIter {
        fn new(iter: Vec<Bucket>) -> Self {
            MyIter { iter }
        }

        fn iter(&self) -> &[Bucket] {
            &self.iter
        }
    }

    impl std::fmt::Debug for MyIter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let iter = self.iter();
            f.debug_list().entries(iter.iter().map(|b| &b.value)).finish()
        }
    }

    let my_iter = MyIter::new(vec![
        Bucket { value: 1 },
        Bucket { value: 2 },
        Bucket { value: 3 },
    ]);
    let result = format!("{:?}", my_iter);
    assert_eq!(result, "[1, 2, 3]");
}

