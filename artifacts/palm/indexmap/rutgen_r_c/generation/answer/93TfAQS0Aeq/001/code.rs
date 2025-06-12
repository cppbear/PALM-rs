// Answer 0

#[test]
fn test_split_last_mut_non_empty() {
    // Create a mutable Slice with more than one entry
    let mut slice = Slice {
        entries: [
            Bucket { hash: 1, key: "first", value: 10 },
            Bucket { hash: 2, key: "second", value: 20 },
            Bucket { hash: 3, key: "last", value: 30 },
        ],
    };

    if let Some((last_pair, rest_slice)) = slice.split_last_mut() {
        // Check that the last item is correct
        assert_eq!(last_pair.0, &"last");
        assert_eq!(*last_pair.1, 30);

        // Check that the rest slice has the correct length
        assert_eq!(rest_slice.len(), 2);
    } else {
        panic!("Expected Some result, but got None");
    }
}

#[test]
fn test_split_last_mut_single_entry() {
    // Create a mutable Slice with a single entry
    let mut slice = Slice {
        entries: [
            Bucket { hash: 1, key: "only", value: 10 },
        ],
    };

    if let Some((last_pair, rest_slice)) = slice.split_last_mut() {
        // Check that the last item is correct
        assert_eq!(last_pair.0, &"only");
        assert_eq!(*last_pair.1, 10);

        // Check that the rest slice is empty
        assert!(rest_slice.is_empty());
    } else {
        panic!("Expected Some result, but got None");
    }
}

#[test]
fn test_split_last_mut_empty() {
    // Create a mutable Slice with no entries
    let mut slice = Slice {
        entries: [],
    };

    // Should return None since the slice is empty
    assert!(slice.split_last_mut().is_none());
}

