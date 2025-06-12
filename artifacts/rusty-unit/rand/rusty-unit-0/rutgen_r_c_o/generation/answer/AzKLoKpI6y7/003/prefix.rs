// Answer 0

#[test]
fn test_partial_shuffle_max_len() {
    let mut vec = Vec::with_capacity(u32::MAX as usize);
    for i in 0..u32::MAX as usize {
        vec.push(i);
    }
    
    let mut rng = rand::thread_rng();
    let amount = 1;
    let (selected, remaining) = vec.partial_shuffle(&mut rng, amount);
    // The assertions are omitted as per the request
}

#[test]
fn test_partial_shuffle_mid_amount() {
    let mut vec = Vec::with_capacity(u32::MAX as usize);
    for i in 0..u32::MAX as usize {
        vec.push(i);
    }
    
    let mut rng = rand::thread_rng();
    let amount = (u32::MAX as usize) / 2;
    let (selected, remaining) = vec.partial_shuffle(&mut rng, amount);
    // The assertions are omitted as per the request
}

#[test]
fn test_partial_shuffle_full_amount() {
    let mut vec = Vec::with_capacity(u32::MAX as usize);
    for i in 0..u32::MAX as usize {
        vec.push(i);
    }
    
    let mut rng = rand::thread_rng();
    let amount = u32::MAX as usize;
    let (selected, remaining) = vec.partial_shuffle(&mut rng, amount);
    // The assertions are omitted as per the request
}

#[test]
fn test_partial_shuffle_zero_amount() {
    let mut vec = Vec::with_capacity(u32::MAX as usize);
    for i in 0..u32::MAX as usize {
        vec.push(i);
    }
    
    let mut rng = rand::thread_rng();
    let amount = 0;
    let (selected, remaining) = vec.partial_shuffle(&mut rng, amount);
    // The assertions are omitted as per the request
}

