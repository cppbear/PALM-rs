// Answer 0

#[test]
fn test_shuffle_normal_case() {
    let mut rng = Rng::with_seed(42);
    let mut slice = [1, 2, 3, 4, 5];
    rng.shuffle(&mut slice);
}

#[test]
fn test_shuffle_large_array() {
    let mut rng = Rng::with_seed(42);
    let mut slice: Vec<_> = (1..=1000).collect();
    rng.shuffle(&mut slice);
}

#[test]
fn test_shuffle_two_elements() {
    let mut rng = Rng::with_seed(42);
    let mut slice = [1, 2];
    rng.shuffle(&mut slice);
}

#[test]
#[should_panic] 
fn test_shuffle_empty_slice() {
    let mut rng = Rng::with_seed(42);
    let mut slice: [i32; 0] = [];
    rng.shuffle(&mut slice);
}

#[test]
#[should_panic] 
fn test_shuffle_single_element() {
    let mut rng = Rng::with_seed(42);
    let mut slice = [1];
    rng.shuffle(&mut slice);
}

