fn sample_floyd<R>(rng: &mut R, length: u32, amount: u32) -> IndexVec
where
    R: Rng + ?Sized,
{
    // Note that the values returned by `rng.random_range()` can be
    // inferred from the returned vector by working backwards from
    // the last entry. This bijection proves the algorithm fair.
    debug_assert!(amount <= length);
    let mut indices = Vec::with_capacity(amount as usize);
    for j in length - amount..length {
        let t = rng.random_range(..=j);
        if let Some(pos) = indices.iter().position(|&x| x == t) {
            indices[pos] = j;
        }
        indices.push(t);
    }
    IndexVec::from(indices)
}