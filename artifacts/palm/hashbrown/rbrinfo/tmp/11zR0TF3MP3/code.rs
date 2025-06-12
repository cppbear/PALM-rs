pub(crate) fn equivalent<Q, K>(k: &Q) -> impl Fn(&K) -> bool + '_
where
    Q: Equivalent<K> + ?Sized,
{
    move |x| k.equivalent(x)
}