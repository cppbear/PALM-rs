// Answer 0

#[test]
fn test_splice_valid_range() {
    use crate::IndexMap;

    let mut map = IndexMap::from([(0, '_'), (1, 'a'), (2, 'b'), (3, 'c'), (4, 'd')]);
    let new = [(5, 'E'), (4, 'D'), (3, 'C'), (2, 'B'), (1, 'A')];
    let removed: Vec<_> = map.splice(2..4, new).collect();

    let expected_map: Vec<_> = vec![(0, '_'), (1, 'A'), (5, 'E'), (3, 'C'), (2, 'B'), (4, 'D')];
    assert!(map.into_iter().eq(expected_map));
    assert_eq!(removed, &[(2, 'b'), (3, 'c')]);
}

#[test]
#[should_panic]
fn test_splice_panic_start_greater_than_end() {
    use crate::IndexMap;

    let mut map = IndexMap::from([(0, '_'), (1, 'a'), (2, 'b')]);
    let new = [(3, 'C')];
    let _: Vec<_> = map.splice(3..1, new).collect();
}

#[test]
#[should_panic]
fn test_splice_panic_end_greater_than_length() {
    use crate::IndexMap;

    let mut map = IndexMap::from([(0, '_'), (1, 'a'), (2, 'b')]);
    let new = [(3, 'C')];
    let _: Vec<_> = map.splice(0..5, new).collect();
}

#[test]
fn test_splice_empty_range() {
    use crate::IndexMap;

    let mut map = IndexMap::from([(0, '_'), (1, 'a'), (2, 'b')]);
    let new = [(3, 'C')];
    let removed: Vec<_> = map.splice(1..1, new).collect(); // Empty range

    let expected_map: Vec<_> = vec![(0, '_'), (1, 'a'), (2, 'b'), (3, 'C')];
    assert!(map.into_iter().eq(expected_map));
    assert!(removed.is_empty());
}

