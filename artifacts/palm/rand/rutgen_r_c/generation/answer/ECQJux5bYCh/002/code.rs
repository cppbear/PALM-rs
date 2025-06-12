// Answer 0

#[test]
fn test_fill_empty_slice() {
    use rand::Rng;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    let mut rng = StdRng::from_seed([0; 32]);
    let mut slice: &mut [u8] = &mut [];
    slice.fill(&mut rng);
    assert!(slice.is_empty());
}

#[test]
fn test_fill_single_element() {
    use rand::Rng;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    let mut rng = StdRng::from_seed([0; 32]);
    let mut slice: &mut [u8] = &mut [0];
    slice.fill(&mut rng);
    assert_ne!(slice[0], 0);
}

#[test]
fn test_fill_multiple_elements() {
    use rand::Rng;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    let mut rng = StdRng::from_seed([0; 32]);
    let mut slice: &mut [u8] = &mut [0; 10];
    slice.fill(&mut rng);
    for &elem in slice.iter() {
        assert_ne!(elem, 0);
    }
}

#[test]
#[should_panic]
fn test_fill_panic_on_empty() {
    use rand::Rng;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    let mut rng = StdRng::from_seed([0; 32]);
    let mut slice: &mut [i32] = &mut [];
    slice.fill(&mut rng); // This will not panic, hence it is not required to check a conditional here.
}

#[test]
fn test_fill_different_types() {
    use rand::Rng;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    let mut rng = StdRng::from_seed([0; 32]);
    let mut slice_u16: &mut [u16] = &mut [0; 5];
    slice_u16.fill(&mut rng);
    for &elem in slice_u16.iter() {
        assert_ne!(elem, 0);
    }

    let mut slice_f32: &mut [f32] = &mut [0.0; 5];
    slice_f32.fill(&mut rng);
    for &elem in slice_f32.iter() {
        assert_ne!(elem, 0.0);
    }
}

