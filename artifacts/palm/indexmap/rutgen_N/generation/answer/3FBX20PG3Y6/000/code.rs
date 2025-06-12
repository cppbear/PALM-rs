// Answer 0

#[test]
fn test_new_index_set_iter() {
    use indexmap::IndexSet;

    let set: IndexSet<i32> = IndexSet::from([1, 2, 3]);
    let other: IndexSet<i32> = IndexSet::from([4, 5, 6]);

    let result = new(&set, &other);
    
    assert_eq!(result.iter.clone().collect::<Vec<_>>(), vec![1, 2, 3]);
    assert_eq!(result.other.clone().collect::<Vec<_>>(), vec![4, 5, 6]);
}

