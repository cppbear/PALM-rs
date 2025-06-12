// Answer 0

#[test]
fn test_iter_with_empty_locations() {
    struct CaptureGroup {
        locs: Vec<usize>,
    }

    impl CaptureGroup {
        fn iter<'c>(&'c self) -> SubCaptureMatches<'c> {
            SubCaptureMatches {
                caps: self,
                it: self.locs.iter(),
            }
        }
    }

    struct SubCaptureMatches<'c> {
        caps: &'c CaptureGroup,
        it: std::slice::Iter<'c, usize>,
    }

    let capture_group = CaptureGroup {
        locs: vec![],
    };

    let mut iter = capture_group.iter();
    assert_eq!(iter.it.clone().count(), 0);
}

#[test]
fn test_iter_with_single_location() {
    struct CaptureGroup {
        locs: Vec<usize>,
    }

    impl CaptureGroup {
        fn iter<'c>(&'c self) -> SubCaptureMatches<'c> {
            SubCaptureMatches {
                caps: self,
                it: self.locs.iter(),
            }
        }
    }

    struct SubCaptureMatches<'c> {
        caps: &'c CaptureGroup,
        it: std::slice::Iter<'c, usize>,
    }

    let capture_group = CaptureGroup {
        locs: vec![1],
    };

    let mut iter = capture_group.iter();
    assert_eq!(iter.it.clone().collect::<Vec<_>>(), vec![&1]);
}

#[test]
fn test_iter_with_multiple_locations() {
    struct CaptureGroup {
        locs: Vec<usize>,
    }

    impl CaptureGroup {
        fn iter<'c>(&'c self) -> SubCaptureMatches<'c> {
            SubCaptureMatches {
                caps: self,
                it: self.locs.iter(),
            }
        }
    }

    struct SubCaptureMatches<'c> {
        caps: &'c CaptureGroup,
        it: std::slice::Iter<'c, usize>,
    }

    let capture_group = CaptureGroup {
        locs: vec![1, 2, 3],
    };

    let mut iter = capture_group.iter();
    assert_eq!(iter.it.clone().collect::<Vec<_>>(), vec![&1, &2, &3]);
}

#[test]
#[should_panic]
fn test_iter_with_panic_condition() {
    struct CaptureGroup {
        locs: Vec<usize>,
    }

    impl CaptureGroup {
        fn iter<'c>(&'c self) -> SubCaptureMatches<'c> {
            SubCaptureMatches {
                caps: self,
                it: self.locs.iter(),
            }
        }
    }

    struct SubCaptureMatches<'c> {
        caps: &'c CaptureGroup,
        it: std::slice::Iter<'c, usize>,
    }

    let capture_group = CaptureGroup {
        locs: vec![0],
    };

    let mut iter = capture_group.iter();
    let _: usize = iter.it.nth(1).unwrap(); // This will panic since there is only one element
}

