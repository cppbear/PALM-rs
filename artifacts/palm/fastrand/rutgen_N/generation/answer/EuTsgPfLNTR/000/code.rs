// Answer 0

#[test]
fn test_choose_multiple_empty_source() {
    let source: Vec<i32> = Vec::new();
    let result = fastrand::choose_multiple(source, 5);
    assert_eq!(result.len(), 0);
}

#[test]
fn test_choose_multiple_small_source() {
    let source = vec![1, 2, 3];
    let result = fastrand::choose_multiple(source.clone(), 2);
    assert_eq!(result.len(), 2);
    for value in result {
        assert!(source.contains(&value));
    }
}

#[test]
fn test_choose_multiple_exactly_one() {
    let source = vec![1];
    let result = fastrand::choose_multiple(source.clone(), 1);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], 1);
}

#[test]
fn test_choose_multiple_more_than_source() {
    let source = vec![1, 2, 3];
    let result = fastrand::choose_multiple(source.clone(), 5);
    assert_eq!(result.len(), 3);
    for value in result {
        assert!(source.contains(&value));
    }
}

#[test]
fn test_choose_multiple_no_item_selection() {
    let source = vec![1, 2, 3];
    let result = fastrand::choose_multiple(source, 0);
    assert_eq!(result.len(), 0);
}

