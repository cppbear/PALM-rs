pub fn append(&mut self, other: &mut Self) {
        #[cfg(feature = "preserve_order")]
        self.map
            .extend(mem::replace(&mut other.map, MapImpl::default()));
        #[cfg(not(feature = "preserve_order"))]
        self.map.append(&mut other.map);
    }