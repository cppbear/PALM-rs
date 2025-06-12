pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(String, Value)>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {
        #[cfg(feature = "preserve_order")]
        return self.swap_remove_entry(key);
        #[cfg(not(feature = "preserve_order"))]
        return self.map.remove_entry(key);
    }