pub fn remove<Q>(&mut self, key: &Q) -> Option<Value>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {
        #[cfg(feature = "preserve_order")]
        return self.swap_remove(key);
        #[cfg(not(feature = "preserve_order"))]
        return self.map.remove(key);
    }