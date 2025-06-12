// Answer 0

#[test]
fn splice_with_valid_range_and_replacement() {
    use crate::IndexMap;

    let mut map = IndexMap::from([(0, '_'), (1, 'a'), (2, 'b'), (3, 'c'), (4, 'd')]);
    let new = [(5, 'E'), (4, 'D'), (3, 'C'), (2, 'B'), (1, 'A')];
    
    let removed: Vec<_> = map.splice(2..4, new).collect();
    
    assert!(map.into_iter().eq([(0, '_'), (1, 'A'), (5, 'E'), (3, 'C'), (2, 'B'), (4, 'D')]));
    assert_eq!(removed, &[(2, 'b'), (3, 'c')]);
}

#[test]
#[should_panic]
fn splice_with_panic_on_invalid_range_start_greater_than_end() {
    let mut map = IndexMap::new();
    map.insert(1, 'a');
    let _ = map.splice(2..1, vec![(2, 'B')]);
}

#[test]
#[should_panic]
fn splice_with_panic_on_range_end_out_of_bounds() {
    let mut map = IndexMap::new();
    map.insert(1, 'a');
    let _ = map.splice(0..2, vec![(2, 'B')]);
}

#[test]
fn splice_with_empty_replace_iterator() {
    let mut map = IndexMap::from([(0, '_'), (1, 'a'), (2, 'b'), (3, 'c'), (4, 'd')]);
    let removed: Vec<_> = map.splice(1..3, vec![]).collect();
    
    assert!(map.into_iter().eq([(0, '_'), (1, 'a'), (3, 'c'), (4, 'd')]));
    assert_eq!(removed, &[(1, 'a'), (2, 'b')]);
}

#[test]
fn splice_with_exact_length_replacement() {
    let mut map = IndexMap::from([(0, '_'), (1, 'a'), (2, 'b'), (3, 'c'), (4, 'd')]);
    let new = [(5, 'E'), (4, 'D')];
    
    let removed: Vec<_> = map.splice(2..4, new).collect();
    
    assert!(map.into_iter().eq([(0, '_'), (1, 'A'), (5, 'E'), (4, 'D'), (2, 'b')]));
    assert_eq!(removed, &[(2, 'b'), (3, 'c')]);
}

#[test]
fn splice_with_single_element_replacement() {
    let mut map = IndexMap::from([(0, '_'), (1, 'a'), (2, 'b'), (3, 'c'), (4, 'd')]);
    let new = [(5, 'E')];
    
    let removed: Vec<_> = map.splice(1..2, new).collect();

    assert!(map.into_iter().eq([(0, '_'), (5, 'E'), (2, 'b'), (3, 'c'), (4, 'd')]));
    assert_eq!(removed, &[(1, 'a')]);
}

