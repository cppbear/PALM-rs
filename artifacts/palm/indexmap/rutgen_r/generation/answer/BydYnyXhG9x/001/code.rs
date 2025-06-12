// Answer 0

#[test]
fn test_retain_keep_all() {
    let mut set = indexmap::IndexSet::new();
    set.insert("a");
    set.insert("b");
    set.insert("c");

    set.retain(|&x| true);

    assert_eq!(set.len(), 3);
    assert!(set.contains("a"));
    assert!(set.contains("b"));
    assert!(set.contains("c"));
}

#[test]
fn test_retain_remove_all() {
    let mut set = indexmap::IndexSet::new();
    set.insert("a");
    set.insert("b");
    set.insert("c");

    set.retain(|&x| false);

    assert_eq!(set.len(), 0);
}

#[test]
fn test_retain_keep_some() {
    let mut set = indexmap::IndexSet::new();
    set.insert("a");
    set.insert("b");
    set.insert("c");

    set.retain(|&x| x != "b");

    assert_eq!(set.len(), 2);
    assert!(set.contains("a"));
    assert!(set.contains("c"));
    assert!(!set.contains("b"));
}

#[test]
fn test_retain_empty_set() {
    let mut set = indexmap::IndexSet::new();

    set.retain(|&x| true);

    assert_eq!(set.len(), 0);
}

#[test]
fn test_retain_with_panic() {
    #[should_panic]
    fn panic_fn(_: &str) -> bool {
        panic!("Intentional panic");
    }

    let mut set = indexmap::IndexSet::new();
    set.insert("a");
    set.insert("b");

    set.retain(panic_fn);
}

