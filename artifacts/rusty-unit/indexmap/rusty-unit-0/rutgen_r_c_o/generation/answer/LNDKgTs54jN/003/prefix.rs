// Answer 0

#[test]
fn test_shift_insert_move_existing_entry() {
    let mut map: IndexMap<char, ()> = ('a'..='d').map(|c| (c, ())).collect();
    let old_value = map.shift_insert(1, 'b', ());
}

#[test]
fn test_shift_insert_move_existing_entry_last() {
    let mut map: IndexMap<char, ()> = ('a'..='d').map(|c| (c, ())).collect();
    let old_value = map.shift_insert(2, 'd', ());
}

#[test]
fn test_shift_insert_move_existing_entry_first() {
    let mut map: IndexMap<char, ()> = ('b'..='e').map(|c| (c, ())).collect();
    let old_value = map.shift_insert(0, 'c', ());
}

#[test]
#[should_panic]
fn test_shift_insert_out_of_bounds_move() {
    let mut map: IndexMap<char, ()> = ('a'..='e').map(|c| (c, ())).collect();
    let _ = map.shift_insert(5, 'b', ());
}

