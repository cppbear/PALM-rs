pub(crate) fn hash<Q: ?Sized + Hash>(&self, key: &Q) -> HashValue {
        let mut h = self.hash_builder.build_hasher();
        key.hash(&mut h);
        HashValue(h.finish() as usize)
    }