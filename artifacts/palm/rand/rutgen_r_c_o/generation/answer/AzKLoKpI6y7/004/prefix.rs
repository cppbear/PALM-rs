// Answer 0

#[test]
fn test_partial_shuffle_max_length() {
    let mut rng = rand::thread_rng();
    let mut slice = vec![0u32; u32::MAX as usize];
    let amount = 0; // m will be 0 since self.len() == (u32::MAX as usize)
    let result = slice.partial_shuffle(&mut rng, amount);
}

#[test]
fn test_partial_shuffle_no_elements() {
    let mut rng = rand::thread_rng();
    let mut slice: Vec<u32> = Vec::new(); // Empty slice
    let amount = 0;
    let result = slice.partial_shuffle(&mut rng, amount);
}

#[test]
fn test_partial_shuffle_amount_equals_length() {
    let mut rng = rand::thread_rng();
    let mut slice = vec![0u32; u32::MAX as usize];
    let amount = u32::MAX as usize; 
    let result = slice.partial_shuffle(&mut rng, amount);
}

#[test]
fn test_partial_shuffle_exceeding_amount() {
    let mut rng = rand::thread_rng();
    let mut slice = vec![0u32; u32::MAX as usize];
    let amount = (u32::MAX as usize) + 1; // Should handle gracefully, as excess
    let result = slice.partial_shuffle(&mut rng, amount);
}

