// Answer 0

#[test]
fn test_num_choices_non_empty_slice() {
    use core::num::NonZeroUsize;
    let slice: &[i32] = &[1, 2, 3];
    let choose = Choose::new(slice).unwrap();
    let count = choose.num_choices();
    assert_eq!(count, NonZeroUsize::new(3).unwrap());
}

#[test]
fn test_num_choices_empty_slice() {
    let slice: &[i32] = &[];
    assert!(Choose::new(slice).is_err());
}

