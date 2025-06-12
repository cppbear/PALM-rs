// Answer 0

#[test]
fn test_new_mut_initializes_empty_slice() {
    let empty_slice: &mut Vec<()> = &mut Vec::new();
    let result = new_mut();
    assert_eq!(result.len(), 0);
}

#[test]
fn test_new_mut_is_mutable() {
    let result = new_mut();
    result.push(());
    assert_eq!(result.len(), 1);
}

