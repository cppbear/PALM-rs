// Answer 0

#[derive(Debug)]
enum ClassSetItem {
    Empty(Option<()>),
    Other,
}

#[derive(Debug)]
enum ClassSet {
    Item(ClassSetItem),
}

#[test]
fn test_is_empty_true() {
    let class_set = ClassSet::Item(ClassSetItem::Empty(Some(())));
    assert_eq!(class_set.is_empty(), true);
}

#[test]
fn test_is_empty_false_other() {
    let class_set = ClassSet::Item(ClassSetItem::Other);
    assert_eq!(class_set.is_empty(), false);
}

#[test]
fn test_is_empty_false_non_empty() {
    let class_set = ClassSet::Item(ClassSetItem::Empty(None));
    assert_eq!(class_set.is_empty(), true);
}

