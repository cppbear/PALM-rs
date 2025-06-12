// Answer 0

#[test]
fn test_iter_with_some_captures() {
    struct MockCaptures {
        locs: Vec<Option<usize>>,
    }

    impl MockCaptures {
        fn new(locs: Vec<Option<usize>>) -> Self {
            Self { locs }
        }

        fn iter(&self) -> std::slice::Iter<Option<usize>> {
            self.locs.iter()
        }
    }

    struct SubCaptureMatches<'c, 't> {
        caps: &'c MockCaptures,
        it: std::slice::Iter<'t, Option<usize>>,
    }

    let captures = MockCaptures::new(vec![Some(1), None, Some(2)]);
    let mut iter = captures.iter();

    assert_eq!(iter.next(), Some(&Some(1)));
    assert_eq!(iter.next(), Some(&None));
    assert_eq!(iter.next(), Some(&Some(2)));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_iter_with_no_captures() {
    struct MockCaptures {
        locs: Vec<Option<usize>>,
    }

    impl MockCaptures {
        fn new(locs: Vec<Option<usize>>) -> Self {
            Self { locs }
        }

        fn iter(&self) -> std::slice::Iter<Option<usize>> {
            self.locs.iter()
        }
    }

    struct SubCaptureMatches<'c, 't> {
        caps: &'c MockCaptures,
        it: std::slice::Iter<'t, Option<usize>>,
    }

    let captures = MockCaptures::new(vec![]);
    let mut iter = captures.iter();

    assert_eq!(iter.next(), None);
}

