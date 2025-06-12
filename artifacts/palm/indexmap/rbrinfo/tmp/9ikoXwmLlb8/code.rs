fn refs(&self) -> (&K, &V) {
        (&self.key, &self.value)
    }