// Answer 0

#[test]
fn test_index_valid_group() {
    use std::collections::HashMap;
    use std::sync::Arc;
    use re_trait::Locations;
    
    struct Slot {
        start: usize,
        end: usize,
    }

    let locs = Locations(vec![Slot { start: 0, end: 4 }, Slot { start: 5, end: 9 }]);
    let named_groups = Arc::new(HashMap::new());
    let text = "Hello World";
    
    let captures = Captures {
        text,
        locs,
        named_groups,
    };

    let group_0 = captures.index(0);
    assert_eq!(group_0, "Hell"); // Assuming this is the intended substring
    let group_1 = captures.index(1);
    assert_eq!(group_1, "o Wo"); // Assuming this is the intended substring
}

#[test]
#[should_panic(expected = "no group at index '2'")]
fn test_index_invalid_group() {
    use std::collections::HashMap;
    use std::sync::Arc;
    use re_trait::Locations;

    struct Slot {
        start: usize,
        end: usize,
    }

    let locs = Locations(vec![Slot { start: 0, end: 4 }, Slot { start: 5, end: 9 }]);
    let named_groups = Arc::new(HashMap::new());
    let text = "Hello World";
    
    let captures = Captures {
        text,
        locs,
        named_groups,
    };

    // This will panic because there are only two groups (0 and 1)
    let _group_2 = captures.index(2);
}

