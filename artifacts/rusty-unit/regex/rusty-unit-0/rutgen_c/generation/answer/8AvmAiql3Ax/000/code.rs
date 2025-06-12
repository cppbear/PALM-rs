// Answer 0

#[test]
fn test_index_valid() {
    use std::collections::HashMap;
    use std::sync::Arc;

    struct Slot {
        start: usize,
        end: usize,
    }
    
    let text: &[u8] = b"test string";
    let locs = Locations(vec![Slot { start: 0, end: 4 }, Slot { start: 5, end: 11 }]);
    let named_groups: Arc<HashMap<String, usize>> = Arc::new(HashMap::new());
    
    let captures = Captures {
        text,
        locs,
        named_groups,
    };

    let segment = captures.index(0);
    assert_eq!(segment, b"test");
}

#[test]
#[should_panic(expected = "no group at index '1'")]
fn test_index_invalid() {
    use std::collections::HashMap;
    use std::sync::Arc;

    struct Slot {
        start: usize,
        end: usize,
    }

    let text: &[u8] = b"test string";
    let locs = Locations(vec![Slot { start: 0, end: 4 }]);
    let named_groups: Arc<HashMap<String, usize>> = Arc::new(HashMap::new());
    
    let captures = Captures {
        text,
        locs,
        named_groups,
    };

    let _segment = captures.index(1);
}

