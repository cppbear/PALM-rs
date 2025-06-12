pub fn from_bounds<I>(iter: &I) -> Option<usize>
where
    I: Iterator,
{
    helper(iter.size_hint())
}