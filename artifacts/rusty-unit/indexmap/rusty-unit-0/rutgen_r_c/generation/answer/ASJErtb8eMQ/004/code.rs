// Answer 0

fn test_get_disjoint_opt_mut_valid_indices() {
    let mut slice = Box::new(Slice { entries: [
        Bucket { hash: 0, key: "a", value: 1 },
        Bucket { hash: 1, key: "b", value: 2 },
        Bucket { hash: 2, key: "c", value: 3 },
    ]});

    let result = slice.get_disjoint_opt_mut([Some(0), Some(1), Some(2)]).unwrap();
    assert_eq!(result[0], Some((&"a", &mut 1)));
    assert_eq!(result[1], Some((&"b", &mut 2)));
    assert_eq!(result[2], Some((&"c", &mut 3)));
}

fn test_get_disjoint_opt_mut_out_of_bounds() {
    let mut slice = Box::new(Slice { entries: [
        Bucket { hash: 0, key: "a", value: 1 },
        Bucket { hash: 1, key: "b", value: 2 },
        Bucket { hash: 2, key: "c", value: 3 },
    ]});
    
    let result = slice.get_disjoint_opt_mut([Some(0), Some(1), Some(3)]);
    assert_eq!(result, Err(GetDisjointMutError::IndexOutOfBounds));
}

fn test_get_disjoint_opt_mut_overlapping_indices() {
    let mut slice = Box::new(Slice { entries: [
        Bucket { hash: 0, key: "a", value: 1 },
        Bucket { hash: 1, key: "b", value: 2 },
        Bucket { hash: 2, key: "c", value: 3 },
    ]});
    
    let result = slice.get_disjoint_opt_mut([Some(0), Some(1), Some(0)]);
    assert_eq!(result, Err(GetDisjointMutError::OverlappingIndices));
}

fn test_get_disjoint_opt_mut_none_indices() {
    let mut slice = Box::new(Slice { entries: [
        Bucket { hash: 0, key: "a", value: 1 },
        Bucket { hash: 1, key: "b", value: 2 },
        Bucket { hash: 2, key: "c", value: 3 },
    ]});

    let result = slice.get_disjoint_opt_mut([None, Some(1), None]).unwrap();
    assert_eq!(result[0], None);
    assert_eq!(result[1], Some((&"b", &mut 2)));
    assert_eq!(result[2], None);
}

fn test_get_disjoint_opt_mut_empty_slice() {
    let mut slice = Box::new(Slice { entries: [] });

    let result = slice.get_disjoint_opt_mut([None, None]);
    assert_eq!(result, Ok([None, None]));
}

