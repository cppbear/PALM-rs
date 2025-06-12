fn sample_inplace<R>(rng: &mut R, length: u32, amount: u32) -> IndexVec
where
    R: Rng + ?Sized,
{
    debug_assert!(amount <= length);
    let mut indices: Vec<u32> = Vec::with_capacity(length as usize);
    indices.extend(0..length);
    for i in 0..amount {
        let j: u32 = rng.random_range(i..length);
        indices.swap(i as usize, j as usize);
    }
    indices.truncate(amount as usize);
    debug_assert_eq!(indices.len(), amount as usize);
    IndexVec::from(indices)
}