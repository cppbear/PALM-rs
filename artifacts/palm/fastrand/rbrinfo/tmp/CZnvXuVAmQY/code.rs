fn with_rng<R>(f: impl FnOnce(&mut Rng) -> R) -> R {
    RNG.with(|rng| {
        let current = rng.replace(Rng(0));

        let mut restore = RestoreOnDrop { rng, current };

        f(&mut restore.current)
    })
}