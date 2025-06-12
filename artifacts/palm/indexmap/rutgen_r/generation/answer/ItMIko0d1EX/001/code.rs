// Answer 0

#[test]
fn test_splice_with_valid_range_and_new_values() {
    use indexmap::IndexMap;

    let mut map = IndexMap::from([(0, '_'), (1, 'a'), (2, 'b'), (3, 'c'), (4, 'd')]);
    let new = [(5, 'E'), (4, 'D'), (3, 'C'), (2, 'B'), (1, 'A')];
    let removed: Vec<_> = map.splice(2..4, new).collect();

    assert!(map.into_iter().eq([(0, '_'), (1, 'A'), (5, 'E'), (3, 'C'), (2, 'B'), (4, 'D')]));
    assert_eq!(removed, &[(2, 'b'), (3, 'c')]);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_splice_with_start_greater_than_end() {
    use indexmap::IndexMap;

    let mut map = IndexMap::from([(0, '_'), (1, 'a'), (2, 'b'), (3, 'c'), (4, 'd')]);
    let new = [(5, 'E')];
    
    let _removed: Vec<_> = map.splice(3..2, new).collect();
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_splice_with_end_greater_than_length() {
    use indexmap::IndexMap;

    let mut map = IndexMap::from([(0, '_'), (1, 'a'), (2, 'b')]);
    let new = [(5, 'E')];

    let _removed: Vec<_> = map.splice(0..5, new).collect();
}

#[test]
fn test_splice_with_empty_replace_with() {
    use indexmap::IndexMap;

    let mut map = IndexMap::from([(0, '_'), (1, 'a'), (2, 'b'), (3, 'c'), (4, 'd')]);
    let new: Vec<(usize, char)> = Vec::new();
    let removed: Vec<_> = map.splice(1..3, new).collect();

    assert!(map.into_iter().eq([(0, '_'), (1, 'c'), (4, 'd')]));
    assert_eq!(removed, &[(1, 'a'), (2, 'b')]);
}

#[test]
fn test_splice_with_non_matching_keys() {
    use indexmap::IndexMap;

    let mut map = IndexMap::from([(0, '_'), (1, 'a'), (2, 'b'), (3, 'c'), (4, 'd')]);
    let new = [(5, 'E'), (6, 'F')];
    let removed: Vec<_> = map.splice(1..3, new).collect();

    assert!(map.into_iter().eq([(0, '_'), (1, 'E'), (4, 'd'), (5, 'E'), (6, 'F')]));
    assert_eq!(removed, &[(1, 'a'), (2, 'b')]);
}

