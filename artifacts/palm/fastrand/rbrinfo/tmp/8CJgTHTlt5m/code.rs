pub fn choice<I>(iter: I) -> Option<I::Item>
where
    I: IntoIterator,
    I::IntoIter: ExactSizeIterator,
{
    with_rng(|r| r.choice(iter))
}