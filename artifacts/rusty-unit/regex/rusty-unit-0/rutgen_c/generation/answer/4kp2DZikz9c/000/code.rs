// Answer 0

#[test]
fn test_iter_empty_locations() {
    struct Match<'t> {
        start: usize,
        end: usize,
        text: &'t [u8],
    }

    let locs = Locations(vec![]);
    let named_groups = Arc::new(HashMap::new());
    let text: &[u8] = b"";
    let caps = Captures {
        text,
        locs,
        named_groups,
    };

    let iter = caps.iter();
    assert_eq!(iter.caps.len(), 0);
}

#[test]
fn test_iter_single_capture() {
    struct Match<'t> {
        start: usize,
        end: usize,
        text: &'t [u8],
    }

    let locs = Locations(vec![Slot { start: 0, end: 1 }]);
    let named_groups = Arc::new(HashMap::new());
    let text: &[u8] = b"a";
    let caps = Captures {
        text,
        locs,
        named_groups,
    };

    let iter = caps.iter();
    assert_eq!(iter.caps.len(), 1);
}

#[test]
fn test_iter_multiple_captures() {
    struct Match<'t> {
        start: usize,
        end: usize,
        text: &'t [u8],
    }

    let locs = Locations(vec![
        Slot { start: 0, end: 1 }, 
        Slot { start: 2, end: 3 }
    ]);
    let named_groups = Arc::new(HashMap::new());
    let text: &[u8] = b"ab";
    let caps = Captures {
        text,
        locs,
        named_groups,
    };

    let iter = caps.iter();
    assert_eq!(iter.caps.len(), 2);
}

#[test]
fn test_iter_none_capture() {
    struct Match<'t> {
        start: usize,
        end: usize,
        text: &'t [u8],
    }

    let locs = Locations(vec![
        Slot { start: 0, end: 1 },
        Slot { start: 0, end: 0 } // No capture
    ]);
    let named_groups = Arc::new(HashMap::new());
    let text: &[u8] = b"a";
    let caps = Captures {
        text,
        locs,
        named_groups,
    };

    let iter = caps.iter();
    assert_eq!(iter.caps.len(), 2);
}

