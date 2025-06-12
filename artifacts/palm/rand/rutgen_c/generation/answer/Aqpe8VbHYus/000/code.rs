// Answer 0

#[test]
fn test_choose_new_non_empty_slice() {
    let slice = [1, 2, 3];
    let choose = Choose::new(&slice);
    assert!(choose.is_ok());
    let instance = choose.unwrap();
    assert_eq!(instance.num_choices(), NonZeroUsize::new(3).unwrap());
}

#[test]
fn test_choose_new_empty_slice() {
    let slice: Vec<i32> = Vec::new();
    let choose = Choose::new(&slice);
    assert!(choose.is_err());
    assert_eq!(choose.unwrap_err(), Empty);
}

#[test]
fn test_choose_new_single_element_slice() {
    let slice = [42];
    let choose = Choose::new(&slice);
    assert!(choose.is_ok());
    let instance = choose.unwrap();
    assert_eq!(instance.num_choices(), NonZeroUsize::new(1).unwrap());
}

