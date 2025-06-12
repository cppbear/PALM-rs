// Answer 0

#[test]
fn test_intersection_basic() {
    let mut set1: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    let mut set2: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };

    for i in 0..100 {
        set1.insert(i);
    }
    for i in 50..150 {
        set2.insert(i);
    }

    let _iter = set1.intersection(&set2);
}

#[test]
fn test_intersection_partial_overlap() {
    let mut set1: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    let mut set2: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };

    for i in 25..75 {
        set1.insert(i);
    }
    for i in 0..50 {
        set2.insert(i);
    }

    let _iter = set1.intersection(&set2);
}

#[test]
fn test_intersection_no_overlap() {
    let mut set1: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    let mut set2: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };

    for i in 0..10 {
        set1.insert(i);
    }
    for i in 10..20 {
        set2.insert(i);
    }

    let _iter = set1.intersection(&set2);
}

#[test]
fn test_intersection_all_overlap() {
    let mut set1: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    let mut set2: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };

    for i in 0..100 {
        set1.insert(i);
        set2.insert(i);
    }

    let _iter = set1.intersection(&set2);
}

#[test]
fn test_intersection_large_sets() {
    let mut set1: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    let mut set2: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };

    for i in 0..1000 {
        set1.insert(i);
    }
    for i in 500..1500 {
        set2.insert(i);
    }

    let _iter = set1.intersection(&set2);
} 

#[test]
fn test_intersection_large_disjoint_sets() {
    let mut set1: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };
    let mut set2: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::new() } };

    for i in 0..1000 {
        set1.insert(i);
    }
    for i in 1000..2000 {
        set2.insert(i);
    }

    let _iter = set1.intersection(&set2);
}

