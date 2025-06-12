// Answer 0

#[test]
#[should_panic]
fn test_insert_before_panic_out_of_bounds_greater() {
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    // The map length is 26, we will try to insert at an index greater than 26, which should panic.
    map.insert_before(27, '*', ());
}

#[test]
#[should_panic]
fn test_insert_before_panic_out_of_bounds_negative() {
    use indexmap::IndexMap;

    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    // The map length is 26, trying a negative index, should also panic.
    map.insert_before(usize::MAX, '*', ());
}

