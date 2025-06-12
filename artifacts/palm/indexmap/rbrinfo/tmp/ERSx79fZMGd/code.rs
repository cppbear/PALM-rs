fn insert_bulk_no_grow<K, V>(indices: &mut Indices, entries: &[Bucket<K, V>]) {
    assert!(indices.capacity() - indices.len() >= entries.len());
    for entry in entries {
        indices.insert_unique(entry.hash.get(), indices.len(), |_| unreachable!());
    }
}