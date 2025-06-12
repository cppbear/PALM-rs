pub fn shuffle<T>(slice: &mut [T]) {
    with_rng(|r| r.shuffle(slice))
}