fn sample_rejection<X: UInt, R>(rng: &mut R, length: X, amount: X) -> IndexVec
where
    R: Rng + ?Sized,
    IndexVec: From<Vec<X>>,
{
    debug_assert!(amount < length);
    #[cfg(feature = "std")]
    let mut cache = HashSet::with_capacity(amount.as_usize());
    #[cfg(not(feature = "std"))]
    let mut cache = BTreeSet::new();
    let distr = Uniform::new(X::zero(), length).unwrap();
    let mut indices = Vec::with_capacity(amount.as_usize());
    for _ in 0..amount.as_usize() {
        let mut pos = distr.sample(rng);
        while !cache.insert(pos) {
            pos = distr.sample(rng);
        }
        indices.push(pos);
    }

    debug_assert_eq!(indices.len(), amount.as_usize());
    IndexVec::from(indices)
}