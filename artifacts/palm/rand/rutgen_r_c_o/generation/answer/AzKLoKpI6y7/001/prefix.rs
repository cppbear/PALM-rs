// Answer 0

#[test]
fn test_partial_shuffle_with_empty_slice() {
    let mut slice: Vec<i32> = vec![];
    let mut rng = rand::thread_rng();
    let amount = 0;
    let _result = slice.partial_shuffle(&mut rng, amount);
}

#[test]
fn test_partial_shuffle_with_single_element() {
    let mut slice: Vec<i32> = vec![1];
    let mut rng = rand::thread_rng();
    let amount = 0;
    let _result = slice.partial_shuffle(&mut rng, amount);
}

#[test]
fn test_partial_shuffle_with_two_elements() {
    let mut slice: Vec<i32> = vec![1, 2];
    let mut rng = rand::thread_rng();
    let amount = 1;
    let _result = slice.partial_shuffle(&mut rng, amount);
}

#[test]
fn test_partial_shuffle_with_three_elements() {
    let mut slice: Vec<i32> = vec![1, 2, 3];
    let mut rng = rand::thread_rng();
    let amount = 2;
    let _result = slice.partial_shuffle(&mut rng, amount);
}

#[test]
fn test_partial_shuffle_with_edge_case() {
    let mut slice: Vec<i32> = (0..u32::MAX as usize).map(|x| x as i32).collect();
    let mut rng = rand::thread_rng();
    let amount = 1;
    let _result = slice.partial_shuffle(&mut rng, amount);
}

#[test]
fn test_partial_shuffle_with_large_slice() {
    let mut slice: Vec<i32> = (0..100).collect();
    let mut rng = rand::thread_rng();
    let amount = 50;
    let _result = slice.partial_shuffle(&mut rng, amount);
}

