// Answer 0

#[test]
fn test_splice_valid_insertion() {
    use indexmap::IndexMap;

    let mut map = IndexMap::from([(0, '_'), (1, 'a'), (2, 'b'), (3, 'c'), (4, 'd')]);
    let new = [(5, 'E'), (4, 'D'), (3, 'C'), (2, 'B'), (1, 'A')];
    let removed: Vec<_> = map.splice(2..4, new).collect();

    assert!(map.into_iter().eq([(0, '_'), (1, 'A'), (5, 'E'), (3, 'C'), (2, 'B'), (4, 'D')]));
    assert_eq!(removed, &[(2, 'b'), (3, 'c')]);
}

#[test]
#[should_panic]
fn test_splice_invalid_range_start_greater_than_end() {
    let mut map = indexmap::IndexMap::new();
    map.splice(3..2, vec![]); // This should panic
}

#[test]
#[should_panic]
fn test_splice_invalid_range_end_greater_than_length() {
    let mut map = indexmap::IndexMap::from([(0, '_'), (1, 'a')]);
    map.splice(0..3, vec![]); // This should panic
}

#[test]
fn test_splice_empty_replace_with() {
    let mut map = IndexMap::from([(0, '_'), (1, 'a')]);
    let removed: Vec<_> = map.splice(0..1, vec![]).collect();

    assert!(map.into_iter().eq([(1, 'a')]));
    assert_eq!(removed, &[(0, '_')]);
}

#[test]
fn test_splice_single_replacement() {
    let mut map = IndexMap::from([(0, '_'), (1, 'a')]);
    let new = [(1, 'A')];
    let removed: Vec<_> = map.splice(0..1, new).collect();

    assert!(map.into_iter().eq([(1, 'A'), (1, 'a')]));
    assert_eq!(removed, &[(0, '_')]);
}

