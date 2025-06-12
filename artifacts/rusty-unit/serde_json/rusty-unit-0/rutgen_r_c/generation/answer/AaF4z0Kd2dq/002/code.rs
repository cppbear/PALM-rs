// Answer 0

#[test]
fn test_next_with_empty_slice() {
    let mut reader = SliceRead {
        slice: &[],
        index: 0,
    };
    let result = reader.next();
    assert_eq!(result, Ok(None));
}

#[test]
fn test_next_with_single_element_slice() {
    let data = [42];
    let mut reader = SliceRead {
        slice: &data,
        index: 0,
    };
    let result = reader.next();
    assert_eq!(result, Ok(Some(42)));
}

#[test]
fn test_next_with_multiple_elements_slice() {
    let data = [1, 2, 3, 4, 5];
    let mut reader = SliceRead {
        slice: &data,
        index: 0,
    };
    
    for expected in &data {
        let result = reader.next();
        assert_eq!(result, Ok(Some(*expected)));
    }
    
    let result = reader.next();
    assert_eq!(result, Ok(None));
}

#[test]
fn test_next_on_last_element() {
    let data = [10, 20, 30];
    let mut reader = SliceRead {
        slice: &data,
        index: 2,
    };
    let result = reader.next();
    assert_eq!(result, Ok(Some(30)));
}

#[test]
fn test_next_on_beyond_last_element() {
    let data = [100, 200, 300];
    let mut reader = SliceRead {
        slice: &data,
        index: 3,
    };
    let result = reader.next();
    assert_eq!(result, Ok(None));
}

