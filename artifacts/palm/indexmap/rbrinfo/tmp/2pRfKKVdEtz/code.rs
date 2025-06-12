fn equivalent<'a, K, V, Q: ?Sized + Equivalent<K>>(
    key: &'a Q,
    entries: &'a [Bucket<K, V>],
) -> impl Fn(&usize) -> bool + 'a {
    move |&i| Q::equivalent(key, &entries[i].key)
}