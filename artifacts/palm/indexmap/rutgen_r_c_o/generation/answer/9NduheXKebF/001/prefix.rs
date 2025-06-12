// Answer 0

#[test]
fn test_retain2_with_all_keep() {
    let mut set: IndexSet<u32, std::collections::hash_map::RandomState> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: std::collections::hash_map::RandomState::new(),
        },
    };

    for i in 1..=50 {
        set.map.insert(i, ());
    }

    set.retain2(|value| true);
}

#[test]
fn test_retain2_with_no_keep() {
    let mut set: IndexSet<u32, std::collections::hash_map::RandomState> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: std::collections::hash_map::RandomState::new(),
        },
    };

    for i in 1..=50 {
        set.map.insert(i, ());
    }

    set.retain2(|value| false);
}

#[test]
fn test_retain2_with_specific_keep() {
    let mut set: IndexSet<u32, std::collections::hash_map::RandomState> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: std::collections::hash_map::RandomState::new(),
        },
    };

    for i in 1..=50 {
        set.map.insert(i, ());
    }

    set.retain2(|value| *value % 2 == 0);
}

#[test]
fn test_retain2_edge_case_empty() {
    let mut set: IndexSet<u32, std::collections::hash_map::RandomState> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: std::collections::hash_map::RandomState::new(),
        },
    };

    set.retain2(|value| true);
}

#[test]
fn test_retain2_with_large_values() {
    let mut set: IndexSet<u64, std::collections::hash_map::RandomState> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: std::collections::hash_map::RandomState::new(),
        },
    };

    for i in 1..=50 {
        set.map.insert(i * 200, ());
    }

    set.retain2(|value| *value < 800);
}

