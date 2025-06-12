// Answer 0

#[test]
fn test_choose_new_empty_slice() {
    let slice: &[i32] = &[];
    let result = Choose::new(slice);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Empty);
}

#[test]
fn test_choose_new_non_empty_slice() {
    let slice: &[i32] = &[1, 2, 3];
    let result = Choose::new(slice);
    assert!(result.is_ok());
    let choose_instance = result.unwrap();
    assert_eq!(choose_instance.num_choices().get(), 3);
}

#[test]
fn test_choose_new_single_element_slice() {
    let slice: &[i32] = &[42];
    let result = Choose::new(slice);
    assert!(result.is_ok());
    let choose_instance = result.unwrap();
    assert_eq!(choose_instance.num_choices().get(), 1);
}

#[test]
fn test_choose_new_large_slice() {
    let slice: Vec<i32> = (0..1000).collect();
    let result = Choose::new(&slice);
    assert!(result.is_ok());
    let choose_instance = result.unwrap();
    assert_eq!(choose_instance.num_choices().get(), 1000);
}

