// Answer 0

#[test]
fn test_capture_names_empty() {
    struct DummyRegex {
        captures: Vec<&'static str>,
    }

    impl DummyRegex {
        fn capture_names(&self) -> std::slice::Iter<&'static str> {
            self.captures.iter()
        }
    }

    struct TestRegex(DummyRegex);

    let regex = TestRegex(DummyRegex { captures: vec![] });
    let capture_names: Vec<_> = regex.0.capture_names().collect();
    assert_eq!(capture_names.len(), 0);
}

#[test]
fn test_capture_names_single() {
    struct DummyRegex {
        captures: Vec<&'static str>,
    }

    impl DummyRegex {
        fn capture_names(&self) -> std::slice::Iter<&'static str> {
            self.captures.iter()
        }
    }

    struct TestRegex(DummyRegex);

    let regex = TestRegex(DummyRegex { captures: vec!["first"] });
    let capture_names: Vec<_> = regex.0.capture_names().collect();
    assert_eq!(capture_names, vec!["first"]);
}

#[test]
fn test_capture_names_multiple() {
    struct DummyRegex {
        captures: Vec<&'static str>,
    }

    impl DummyRegex {
        fn capture_names(&self) -> std::slice::Iter<&'static str> {
            self.captures.iter()
        }
    }

    struct TestRegex(DummyRegex);

    let regex = TestRegex(DummyRegex { captures: vec!["first", "second", "third"] });
    let capture_names: Vec<_> = regex.0.capture_names().collect();
    assert_eq!(capture_names, vec!["first", "second", "third"]);
}

#[test]
#[should_panic]
fn test_capture_names_panic() {
    struct DummyRegex {
        captures: Vec<&'static str>,
    }

    impl DummyRegex {
        fn capture_names(&self) -> std::slice::Iter<&'static str> {
            self.captures.iter()
        }
    }

    struct TestRegex(DummyRegex);

    let regex = TestRegex(DummyRegex { captures: vec!["panic"] });
    let mut iter = regex.0.capture_names();
    assert_eq!(iter.next(), Some(&"panic"));
    assert_eq!(iter.next(), Some(&"does_not_exist")); // This should cause a panic if there's an assertion here
}

