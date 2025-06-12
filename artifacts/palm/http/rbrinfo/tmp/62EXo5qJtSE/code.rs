fn hash_elem_using<K>(danger: &Danger, k: &K) -> HashValue
where
    K: Hash + ?Sized,
{
    use fnv::FnvHasher;

    const MASK: u64 = (MAX_SIZE as u64) - 1;

    let hash = match *danger {
        // Safe hash
        Danger::Red(ref hasher) => {
            let mut h = hasher.build_hasher();
            k.hash(&mut h);
            h.finish()
        }
        // Fast hash
        _ => {
            let mut h = FnvHasher::default();
            k.hash(&mut h);
            h.finish()
        }
    };

    HashValue((hash & MASK) as u16)
}