fn clone(&self) -> Self {
        MapDeserializer {
            iter: self.iter.clone(),
            value: self.value.clone(),
            count: self.count,
            lifetime: self.lifetime,
            error: self.error,
        }
    }