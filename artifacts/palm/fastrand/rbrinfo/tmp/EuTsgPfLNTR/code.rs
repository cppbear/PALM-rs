pub fn choose_multiple<I: IntoIterator>(source: I, amount: usize) -> Vec<I::Item> {
    with_rng(|rng| rng.choose_multiple(source, amount))
}