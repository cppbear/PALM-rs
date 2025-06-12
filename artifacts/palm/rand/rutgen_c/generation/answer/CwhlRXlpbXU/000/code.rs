// Answer 0

#[test]
fn test_append_string_empty() {
    use crate::Rng;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    let mut rng = StdRng::from_seed([0; 32]);
    let mut string = String::new();
    let dist = StandardUniform;
    
    dist.append_string(&mut rng, &mut string, 0);
    
    assert_eq!(string.len(), 0);
}

#[test]
fn test_append_string_length_one() {
    use crate::Rng;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    let mut rng = StdRng::from_seed([0; 32]);
    let mut string = String::new();
    let dist = StandardUniform;
    
    dist.append_string(&mut rng, &mut string, 1);
    
    assert_eq!(string.len(), 1);
}

#[test]
fn test_append_string_length_ten() {
    use crate::Rng;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    let mut rng = StdRng::from_seed([0; 32]);
    let mut string = String::new();
    let dist = StandardUniform;
    
    dist.append_string(&mut rng, &mut string, 10);
    
    assert_eq!(string.len(), 10);
}

#[test]
fn test_append_string_reserve_capacity() {
    use crate::Rng;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    let mut rng = StdRng::from_seed([0; 32]);
    let mut string = String::new();
    let dist = StandardUniform;

    let len = 50;
    dist.append_string(&mut rng, &mut string, len);
    
    // Making sure the reserved capacity is sufficient
    assert!(string.capacity() >= 4 * len);
}

