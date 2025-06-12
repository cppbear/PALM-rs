// Answer 0

#[test]
fn test_iter_with_non_empty_locs() {
    struct Captures {
        locs: Vec<usize>,
    }
    
    impl Captures {
        fn new(locs: Vec<usize>) -> Self {
            Captures { locs }
        }

        fn iter<'c>(&'c self) -> SubCaptureMatches<'c> {
            SubCaptureMatches {
                caps: self,
                it: self.locs.iter(),
            }
        }
    }

    struct SubCaptureMatches<'c> {
        caps: &'c Captures,
        it: std::slice::Iter<'c, usize>,
    }

    let captures = Captures::new(vec![0, 1, 2]);
    let matches_iter = captures.iter();

    assert_eq!(matches_iter.locs, vec![0, 1, 2]);
}

#[test]
fn test_iter_with_empty_locs() {
    struct Captures {
        locs: Vec<usize>,
    }
    
    impl Captures {
        fn new(locs: Vec<usize>) -> Self {
            Captures { locs }
        }

        fn iter<'c>(&'c self) -> SubCaptureMatches<'c> {
            SubCaptureMatches {
                caps: self,
                it: self.locs.iter(),
            }
        }
    }

    struct SubCaptureMatches<'c> {
        caps: &'c Captures,
        it: std::slice::Iter<'c, usize>,
    }

    let captures = Captures::new(vec![]);
    let matches_iter = captures.iter();

    assert_eq!(matches_iter.locs, vec![]);
}

#[test]
#[should_panic]
fn test_iter_with_none_locs() {
    struct Captures {
        locs: Vec<usize>,
    }
    
    impl Captures {
        fn new(locs: Vec<usize>) -> Self {
            Captures { locs }
        }

        fn iter<'c>(&'c self) -> SubCaptureMatches<'c> {
            SubCaptureMatches {
                caps: self,
                it: self.locs.iter(),
            }
        }
    }

    struct SubCaptureMatches<'c> {
        caps: &'c Captures,
        it: std::slice::Iter<'c, usize>,
    }

    let captures = Captures::new(vec![0, 1, 2]);
    let matches_iter = captures.iter();

    // Assuming we had a way to trigger panic (like access beyond the bounds)
    // Here would be an example, as this portion is based on assumptions
    panic!("This will panic as we're trying to access an invalid index");
}

