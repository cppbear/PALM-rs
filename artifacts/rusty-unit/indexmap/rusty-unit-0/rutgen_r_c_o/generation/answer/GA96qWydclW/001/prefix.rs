// Answer 0

#[test]
fn test_contains_empty_set() {
    let set: IndexSet<i32, RandomState> = IndexSet::new();
    let value: Option<&i32> = None;
    set.contains(value);
}

#[test]
fn test_contains_value_present() {
    let mut set: IndexSet<i32, RandomState> = IndexSet::new();
    set.insert(1);
    let value: &i32 = &1;
    set.contains(value);
}

#[test]
fn test_contains_value_absent() {
    let mut set: IndexSet<i32, RandomState> = IndexSet::new();
    set.insert(1);
    let value: &i32 = &2;
    set.contains(value);
}

#[test]
fn test_contains_with_null_value() {
    let mut set: IndexSet<i32, RandomState> = IndexSet::new();
    let value: Option<&i32> = None;
    set.contains(value);
}

#[test]
fn test_contains_large_set() {
    let mut set: IndexSet<i32, RandomState> = IndexSet::new();
    for i in 0..1000 {
        set.insert(i);
    }
    let value: &i32 = &500;
    set.contains(value);
}

#[test]
fn test_contains_non_hashable_value() {
    struct NonHashable;
    let mut set: IndexSet<i32, RandomState> = IndexSet::new();
    let value: NonHashable = NonHashable;
    set.contains(&value);
}

