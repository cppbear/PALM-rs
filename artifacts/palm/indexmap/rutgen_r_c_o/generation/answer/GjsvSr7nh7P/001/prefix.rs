// Answer 0

#[test]
fn test_shift_insert_move_existing_value() {
    let mut set: IndexSet<char, RandomState> = ('a'..='z').collect();
    set.shift_insert(10, 'a');
    set.shift_insert(9, 'b');
    set.shift_insert(5, 'c');
}

#[test]
fn test_shift_insert_insert_new_value() {
    let mut set: IndexSet<char, RandomState> = ('a'..='z').collect();
    set.shift_insert(10, '*');
    set.shift_insert(27, '+');
}

#[test]
#[should_panic]
fn test_shift_insert_panic_move_existing_value_out_of_bounds() {
    let mut set: IndexSet<char, RandomState> = ('a'..='z').collect();
    set.shift_insert(set.len(), 'a');
}

#[test]
fn test_shift_insert_existing_value_edge_case() {
    let mut set: IndexSet<char, RandomState> = ('a'..='z').collect();
    set.shift_insert(26, 'z');
    set.shift_insert(25, 'y');
}

#[test]
fn test_shift_insert_insert_at_start() {
    let mut set: IndexSet<char, RandomState> = ('a'..='z').collect();
    set.shift_insert(0, '#');
}

#[test]
fn test_shift_insert_shift_multiple_existing_values() {
    let mut set: IndexSet<char, RandomState> = ('a'..='z').collect();
    set.shift_insert(5, 'e');
    set.shift_insert(15, 'n');
}

#[test]
fn test_shift_insert_double_panic() {
    let mut set: IndexSet<char, RandomState> = ('a'..='z').collect();
    set.shift_insert(set.len(), '*');
    set.shift_insert(set.len(), '+');
}

