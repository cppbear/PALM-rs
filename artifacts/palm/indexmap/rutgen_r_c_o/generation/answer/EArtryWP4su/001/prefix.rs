// Answer 0

#[test]
fn test_sub_empty_sets() {
    let set1: IndexSet<i32, RandomState> = IndexSet::default();
    let set2: IndexSet<i32, RandomState> = IndexSet::default();
    let _result = set1.sub(&set2);
}

#[test]
fn test_sub_non_empty_set() {
    let mut set1: IndexSet<i32, RandomState> = IndexSet::default();
    let mut set2: IndexSet<i32, RandomState> = IndexSet::default();

    for i in 1..=10 {
        set1.insert(i);
    }
    
    for i in 5..=15 {
        set2.insert(i);
    }

    let _result = set1.sub(&set2);
}

#[test]
fn test_sub_identical_sets() {
    let mut set1: IndexSet<i32, RandomState> = IndexSet::default();
    let mut set2: IndexSet<i32, RandomState> = IndexSet::default();

    for i in 1..=10 {
        set1.insert(i);
        set2.insert(i);
    }

    let _result = set1.sub(&set2);
}

#[test]
fn test_sub_disjoint_sets() {
    let mut set1: IndexSet<i32, RandomState> = IndexSet::default();
    let mut set2: IndexSet<i32, RandomState> = IndexSet::default();

    for i in 1..=10 {
        set1.insert(i);
    }

    for i in 11..=20 {
        set2.insert(i);
    }

    let _result = set1.sub(&set2);
}

#[test]
fn test_sub_large_sets() {
    let mut set1: IndexSet<i32, RandomState> = IndexSet::default();
    let mut set2: IndexSet<i32, RandomState> = IndexSet::default();

    for i in 1..=100 {
        set1.insert(i);
    }

    for i in 50..=150 {
        set2.insert(i);
    }

    let _result = set1.sub(&set2);
}

#[test]
fn test_sub_multiple_elements() {
    let mut set1: IndexSet<i32, RandomState> = IndexSet::default();
    let mut set2: IndexSet<i32, RandomState> = IndexSet::default();
    
    for i in 1..=20 {
        set1.insert(i);
    }

    for i in 10..=30 {
        set2.insert(i);
    }

    let _result = set1.sub(&set2);
}

