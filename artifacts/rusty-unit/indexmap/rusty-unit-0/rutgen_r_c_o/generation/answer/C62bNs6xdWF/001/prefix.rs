// Answer 0

#[test]
fn test_swap_remove_index_valid_index_first() {
    let mut map: IndexMapCore<usize, String> = IndexMapCore::new();
    map.push_entry(1, 0, String::from("zero"));
    map.push_entry(2, 1, String::from("one"));
    map.swap_remove_index(0);
}

#[test]
fn test_swap_remove_index_valid_index_last() {
    let mut map: IndexMapCore<usize, String> = IndexMapCore::new();
    map.push_entry(1, 0, String::from("zero"));
    map.push_entry(2, 1, String::from("one"));
    map.swap_remove_index(1);
}

#[test]
fn test_swap_remove_index_valid_index_middle() {
    let mut map: IndexMapCore<usize, String> = IndexMapCore::new();
    map.push_entry(1, 0, String::from("zero"));
    map.push_entry(2, 1, String::from("one"));
    map.push_entry(3, 2, String::from("two"));
    map.swap_remove_index(1);
}

#[test]
#[should_panic]
fn test_swap_remove_index_out_of_bounds_high() {
    let mut map: IndexMapCore<usize, String> = IndexMapCore::new();
    map.push_entry(1, 0, String::from("zero"));
    map.push_entry(2, 1, String::from("one"));
    map.swap_remove_index(3);
}

#[test]
#[should_panic]
fn test_swap_remove_index_out_of_bounds_low() {
    let mut map: IndexMapCore<usize, String> = IndexMapCore::new();
    map.push_entry(1, 0, String::from("zero"));
    map.push_entry(2, 1, String::from("one"));
    map.swap_remove_index(usize::MAX);
}

#[test]
fn test_swap_remove_index_empty() {
    let mut map: IndexMapCore<usize, String> = IndexMapCore::new();
    let result = map.swap_remove_index(0);
}

