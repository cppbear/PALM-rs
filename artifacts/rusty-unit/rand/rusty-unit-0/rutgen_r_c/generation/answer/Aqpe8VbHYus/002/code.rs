// Answer 0

#[test]
fn test_choose_new_with_non_empty_slice() {
    let slice = &[1, 2, 3, 4, 5];
    let result = Choose::new(slice);
    assert!(result.is_ok());

    if let Ok(choose_instance) = result {
        assert_eq!(choose_instance.num_choices().get(), 5);
    }
}

#[test]
fn test_choose_new_with_empty_slice() {
    let slice: &[i32] = &[];
    let result = Choose::new(slice);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Empty);
}

#[test]
fn test_choose_new_with_one_element_slice() {
    let slice = &[42];
    let result = Choose::new(slice);
    assert!(result.is_ok());

    if let Ok(choose_instance) = result {
        assert_eq!(choose_instance.num_choices().get(), 1);
    }
}

