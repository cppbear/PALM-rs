// Answer 0

#[test]
fn test_reserve_capacity() {
    use crate::HashSet;
    use crate::DefaultHashBuilder;
    use crate::Global;

    let mut set: HashSet<i32, DefaultHashBuilder, Global> = HashSet::new();
    set.reserve(10);
    assert!(set.map.table.capacity() >= 10);
}

#[test]
#[should_panic(expected = "capacity overflow")]
fn test_reserve_capacity_panics_on_overflow() {
    use crate::HashSet;
    use crate::DefaultHashBuilder;
    use crate::Global;

    let mut set: HashSet<i32, DefaultHashBuilder, Global> = HashSet::new();
    set.reserve(isize::MAX as usize);
}

