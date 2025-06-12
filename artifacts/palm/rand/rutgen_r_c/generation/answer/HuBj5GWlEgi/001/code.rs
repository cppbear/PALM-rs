// Answer 0

#[test]
fn test_num_choices_non_empty() {
    use core::num::NonZeroUsize;
    use crate::distr::uniform::UniformUsize;

    let slice = &[1, 2, 3];
    let choose = Choose::new(slice).unwrap();
    let num_choices = choose.num_choices();
    assert_eq!(num_choices, NonZeroUsize::new(3).unwrap());
}

#[test]
#[should_panic(expected = "Empty")]
fn test_num_choices_empty() {
    use crate::distr::uniform::UniformUsize;

    let slice: &[i32] = &[];
    let _choose = Choose::new(slice).unwrap(); // This should panic
}

