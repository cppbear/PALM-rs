// Answer 0

#[test]
fn test_shift_insert_move_existing_entry() {
    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    map.shift_insert(10, 'a', ());
}

#[test]
fn test_shift_insert_move_existing_entry_to_end() {
    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    map.shift_insert(map.len() - 1, 'b', ());
}

#[test]
fn test_shift_insert_insert_new_entry_at_end() {
    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    map.shift_insert(map.len(), '*', ());
}

#[test]
#[should_panic]
fn test_shift_insert_invalid_move_index() {
    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    map.shift_insert(map.len(), 'a', ());
}

#[test]
#[should_panic]
fn test_shift_insert_out_of_bounds_existing_entry() {
    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    map.shift_insert(30, 'a', ());
}

#[test]
#[should_panic]
fn test_shift_insert_out_of_bounds_new_entry() {
    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    map.shift_insert(30, '#', ());
}

