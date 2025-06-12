// Answer 0

#[test]
fn test_reverse_empty_set() {
    struct TestSet {
        map: super::IndexMap<i32, (), RandomState>,
    }

    let mut set = TestSet { map: super::IndexMap::new() };
    set.reverse();
    // Assert that the set is still empty after reversing
    assert_eq!(set.map.len(), 0);
}

#[test]
fn test_reverse_single_element_set() {
    struct TestSet {
        map: super::IndexMap<i32, (), RandomState>,
    }

    let mut set = TestSet { map: super::IndexMap::new() };
    set.map.insert(1, ());
    set.reverse();
    // Assert that the element is still there after reversing
    assert_eq!(set.map.len(), 1);
    assert_eq!(set.map.get(&1), Some(&()));
}

#[test]
fn test_reverse_multiple_elements_set() {
    struct TestSet {
        map: super::IndexMap<i32, (), RandomState>,
    }

    let mut set = TestSet { map: super::IndexMap::new() };
    set.map.insert(1, ());
    set.map.insert(2, ());
    set.map.insert(3, ());
    
    // Assert initial order
    let initial_order: Vec<_> = set.map.keys().cloned().collect();
    assert_eq!(initial_order, vec![1, 2, 3]);

    set.reverse();

    // Assert order after reversing
    let reversed_order: Vec<_> = set.map.keys().cloned().collect();
    assert_eq!(reversed_order, vec![3, 2, 1]);
}

