fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write(self.0.as_bytes())
    }