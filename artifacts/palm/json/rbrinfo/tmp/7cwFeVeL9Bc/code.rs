fn hash<H: Hasher>(&self, state: &mut H) {
        #[cfg(not(feature = "preserve_order"))]
        {
            self.map.hash(state);
        }

        #[cfg(feature = "preserve_order")]
        {
            let mut kv = Vec::from_iter(&self.map);
            kv.sort_unstable_by(|a, b| a.0.cmp(b.0));
            kv.hash(state);
        }
    }