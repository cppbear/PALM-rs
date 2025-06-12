// Answer 0

#[test]
fn test_is_subset_false_due_to_length() {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    
    let mut sup = HashSet::new();
    sup.insert(1);
    sup.insert(2);
    sup.insert(3);
    sup.insert(4);
    sup.insert(5);
    sup.insert(6);
    
    let result = set.is_subset(&sup);
}

#[test]
fn test_is_subset_false_due_to_missing_elements() {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    
    let mut sup = HashSet::new();
    sup.insert(1);
    sup.insert(3);
    sup.insert(4);
    
    let result = set.is_subset(&sup);
}

#[test]
fn test_is_subset_true_with_equal_elements() {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    
    let mut sup = HashSet::new();
    sup.insert(1);
    sup.insert(2);
    
    let result = set.is_subset(&sup);
}

#[test]
fn test_is_subset_true_with_extra_elements_in_superset() {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    
    let mut sup = HashSet::new();
    sup.insert(1);
    sup.insert(2);
    sup.insert(3);
    
    let result = set.is_subset(&sup);
}

