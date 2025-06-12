// Answer 0

#[test]
fn test_split_last_mut_empty() {
    // Create an empty Slice
    let mut slice: Slice<i32, i32> = Slice { entries: [] };

    // Call the split_last_mut function
    let result = slice.split_last_mut();

    // Assert that the result is None
    assert!(result.is_none());
}

#[test]
fn test_split_last_mut_single_element() {
    // Create a Slice with a single element
    let mut entry = Bucket { hash: 0, key: 1, value: 2 };
    let mut slice: Slice<i32, i32> = Slice { entries: [entry] };

    // Call the split_last_mut function
    let result = slice.split_last_mut();

    // Assert that the result is Some and contains the correct value
    if let Some((last, rest)) = result {
        assert_eq!(*last.0, 1);
        assert_eq!(*last.1, 2);
        assert!(rest.is_empty());
    } else {
        panic!("Expected Some value but got None");
    }
}

#[test]
fn test_split_last_mut_multiple_elements() {
    // Create a Slice with multiple elements
    let entry1 = Bucket { hash: 0, key: 1, value: 2 };
    let entry2 = Bucket { hash: 1, key: 3, value: 4 };
    let mut slice: Slice<i32, i32> = Slice { entries: [entry1, entry2] };

    // Call the split_last_mut function
    let result = slice.split_last_mut();

    // Assert that the result is Some and contains the correct last element
    if let Some((last, rest)) = result {
        assert_eq!(*last.0, 3);
        assert_eq!(*last.1, 4);
        assert_eq!(rest.len(), 1);
        assert_eq!(*rest.get_index(0).unwrap().0, 1);
        assert_eq!(*rest.get_index(0).unwrap().1, 2);
    } else {
        panic!("Expected Some value but got None");
    }
}

