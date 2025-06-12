pub fn rng() -> ThreadRng {
    let rng = THREAD_RNG_KEY.with(|t| t.clone());
    ThreadRng { rng }
}