// Answer 0

#[test]
fn test_splice_with_valid_range_and_new_entries() {
    let mut map = IndexMap::from([(0, '_'), (1, 'a'), (2, 'b'), (3, 'c'), (4, 'd')]);
    let new = vec![(5, 'E'), (4, 'D'), (3, 'C'), (2, 'B'), (1, 'A')];
    let _removed: Vec<_> = map.splice(2..4, new).collect();
}

#[test]
fn test_splice_with_zero_length_range() {
    let mut map = IndexMap::from([(0, '_'), (1, 'a'), (2, 'b')]);
    let new = vec![(3, 'E')];
    let _removed: Vec<_> = map.splice(0..0, new).collect();
}

#[test]
#[should_panic]
fn test_splice_with_start_greater_than_end() {
    let mut map = IndexMap::from([(0, '_'), (1, 'a'), (2, 'b')]);
    let new = vec![(3, 'E')];
    let _removed: Vec<_> = map.splice(2..0, new).collect();
}

#[test]
#[should_panic]
fn test_splice_with_end_greater_than_length() {
    let mut map = IndexMap::from([(0, '_'), (1, 'a'), (2, 'b')]);
    let new = vec![(3, 'E')];
    let _removed: Vec<_> = map.splice(0..10, new).collect();
}

#[test]
fn test_splice_with_exclusive_range() {
    let mut map = IndexMap::from([(0, 'a'), (1, 'b'), (2, 'c'), (3, 'd')]);
    let new = vec![(4, 'E'), (5, 'F')];
    let _removed: Vec<_> = map.splice(1..3, new).collect();
}

#[test]
fn test_splice_with_empty_replace() {
    let mut map = IndexMap::from([(0, '_'), (1, 'a'), (2, 'b')]);
    let new: Vec<(i32, char)> = vec![];
    let _removed: Vec<_> = map.splice(0..3, new).collect();
}

#[test]
fn test_splice_with_overlapping_keys() {
    let mut map = IndexMap::from([(0, '_'), (1, 'a'), (2, 'b'), (3, 'c')]);
    let new = vec![(2, 'D'), (3, 'E'), (4, 'F')];
    let _removed: Vec<_> = map.splice(1..3, new).collect();
}

#[test]
fn test_splice_with_no_removals() {
    let mut map = IndexMap::from([(0, 'X'), (1, 'Y'), (2, 'Z')]);
    let new = vec![(3, 'A')];
    let _removed: Vec<_> = map.splice(2..2, new).collect();
}

#[test]
fn test_splice_with_full_range() {
    let mut map = IndexMap::from([(0, 'a'), (1, 'b'), (2, 'c'), (3, 'd')]);
    let new = vec![(4, 'E'), (5, 'F')];
    let _removed: Vec<_> = map.splice(0..4, new).collect();
}

#[test]
fn test_splice_with_edge_case_of_same_length() {
    let mut map = IndexMap::from([(0, '1'), (1, '2'), (2, '3')]);
    let new = vec![(3, 'A'), (4, 'B'), (5, 'C')];
    let _removed: Vec<_> = map.splice(0..3, new).collect();
}

