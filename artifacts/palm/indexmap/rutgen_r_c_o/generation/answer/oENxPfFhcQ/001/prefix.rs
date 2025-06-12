// Answer 0

#[test]
fn test_new_with_valid_range_and_non_empty_iterator() {
    let mut map: IndexMap<i32, i32, std::collections::hash_map::RandomState> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    let replace_with = vec![(4, 40), (5, 50)].into_iter();

    let _splice = Splice::new(&mut map, 0..2, replace_with);
}

#[test]
fn test_new_with_exact_length_range_and_non_empty_iterator() {
    let mut map: IndexMap<i32, i32, std::collections::hash_map::RandomState> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    let replace_with = vec![(3, 30)].into_iter();

    let _splice = Splice::new(&mut map, 0..1, replace_with);
}

#[test]
fn test_new_with_full_range_and_non_empty_iterator() {
    let mut map: IndexMap<i32, i32, std::collections::hash_map::RandomState> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    let replace_with = vec![(4, 40), (5, 50)].into_iter();

    let _splice = Splice::new(&mut map, .., replace_with);
}

#[test]
fn test_new_with_empty_map_and_range() {
    let mut map: IndexMap<i32, i32, std::collections::hash_map::RandomState> = IndexMap::new();
    let replace_with = vec![(1, 10)].into_iter();

    let _splice = Splice::new(&mut map, 0..0, replace_with);
}

#[should_panic]
fn test_new_with_invalid_range_exceeding_map_size() {
    let mut map: IndexMap<i32, i32, std::collections::hash_map::RandomState> = IndexMap::new();
    map.insert(1, 10);
    let replace_with = vec![(2, 20)].into_iter();

    let _splice = Splice::new(&mut map, 1..3, replace_with);
}

