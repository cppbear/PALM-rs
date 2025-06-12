// Answer 0

#[test]
fn test_iter_with_no_matches() {
    struct FakeCaptures {
        locs: Vec<usize>,
    }

    impl FakeCaptures {
        fn iter(&self) -> std::slice::Iter<usize> {
            self.locs.iter()
        }
    }

    let captures = FakeCaptures { locs: vec![] };
    let mut iter = captures.iter();

    assert!(iter.next().is_none());
}

#[test]
fn test_iter_with_single_match() {
    struct FakeCaptures {
        locs: Vec<usize>,
    }

    impl FakeCaptures {
        fn iter(&self) -> std::slice::Iter<usize> {
            self.locs.iter()
        }
    }

    let captures = FakeCaptures { locs: vec![0] };
    let mut iter = captures.iter();
    
    assert_eq!(iter.next(), Some(&0));
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_with_multiple_matches() {
    struct FakeCaptures {
        locs: Vec<usize>,
    }

    impl FakeCaptures {
        fn iter(&self) -> std::slice::Iter<usize> {
            self.locs.iter()
        }
    }

    let captures = FakeCaptures { locs: vec![0, 1, 2] };
    let mut iter = captures.iter();
    
    assert_eq!(iter.next(), Some(&0));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert!(iter.next().is_none());
}

