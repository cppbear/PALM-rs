// Answer 0

#[test]
fn test_split_off_middle() {
    let mut set = indexmap::IndexSet::new();
    set.insert("a");
    set.insert("b");
    set.insert("c");
    
    let result = set.split_off(1);
    
    assert_eq!(result.len(), 2);
    assert!(result.contains("b"));
    assert!(result.contains("c"));
    assert_eq!(set.len(), 1);
    assert!(set.contains("a"));
}

#[test]
fn test_split_off_empty_set() {
    let mut set = indexmap::IndexSet::new();
    
    let result = set.split_off(0);
    
    assert_eq!(result.len(), 0);
    assert_eq!(set.len(), 0);
}

#[test]
fn test_split_off_at_end() {
    let mut set = indexmap::IndexSet::new();
    set.insert("x");
    set.insert("y");
    set.insert("z");
    
    let result = set.split_off(3);
    
    assert_eq!(result.len(), 0);
    assert_eq!(set.len(), 3);
}

#[should_panic]
fn test_split_off_at_greater_than_length() {
    let mut set = indexmap::IndexSet::new();
    set.insert("foo");
    
    // This should panic since at (5) is greater than the length (1)
    let _ = set.split_off(5);
} 

#[test]
fn test_split_off_exact_length() {
    let mut set = indexmap::IndexSet::new();
    set.insert("1");
    set.insert("2");
    set.insert("3");
    
    let result = set.split_off(3);
    
    assert_eq!(result.len(), 0);
    assert_eq!(set.len(), 3);
}

