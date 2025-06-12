// Answer 0

#[test]
fn test_fmt_with_non_empty_set() {
    struct MySet {
        iter: Vec<(i32, ())>,
    }

    impl MySet {
        fn new() -> Self {
            Self { iter: Vec::new() }
        }
        
        fn add(&mut self, value: i32) {
            self.iter.push((value, ()));
        }
    }

    let mut set = MySet::new();
    set.add(1);
    set.add(2);
    set.add(3);

    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", &set);

    assert!(result.is_ok());
    assert_eq!(buffer, "[1, 2, 3]");
}

#[test]
fn test_fmt_with_empty_set() {
    struct MySet {
        iter: Vec<(i32, ())>,
    }

    impl MySet {
        fn new() -> Self {
            Self { iter: Vec::new() }
        }
    }

    let set = MySet::new();

    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", &set);

    assert!(result.is_ok());
    assert_eq!(buffer, "[]");
}

