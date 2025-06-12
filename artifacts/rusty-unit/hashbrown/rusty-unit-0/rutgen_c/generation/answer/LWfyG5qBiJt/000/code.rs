// Answer 0

#[test]
fn test_take_existing_value() {
    use crate::hashbrown::HashSet;

    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    
    assert_eq!(set.take(&2), Some(2));
    assert_eq!(set.take(&2), None);
}

#[test]
fn test_take_non_existent_value() {
    use crate::hashbrown::HashSet;

    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(3);
    
    assert_eq!(set.take(&2), None);
}

#[test]
fn test_take_multiple_types() {
    use crate::hashbrown::HashSet;

    let mut set: HashSet<String> = HashSet::new();
    set.insert("hello".to_string());
    set.insert("world".to_string());
    
    assert_eq!(set.take(&"hello".to_string()), Some("hello".to_string()));
    assert_eq!(set.take(&"hello".to_string()), None);
    assert_eq!(set.take(&"world".to_string()), Some("world".to_string()));
}

#[test]
fn test_take_with_different_borrowed_form() {
    use crate::hashbrown::HashSet;

    let mut set: HashSet<&str> = HashSet::new();
    set.insert("rust");
    set.insert("is");
    set.insert("awesome");

    assert_eq!(set.take(&"is"), Some("is"));
    assert_eq!(set.take(&"is"), None);
}

