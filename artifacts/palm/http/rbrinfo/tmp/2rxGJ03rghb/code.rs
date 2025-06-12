fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        if !self.scheme.inner.is_none() {
            self.scheme.hash(state);
            state.write_u8(0xff);
        }

        if let Some(auth) = self.authority() {
            auth.hash(state);
        }

        Hash::hash_slice(self.path().as_bytes(), state);

        if let Some(query) = self.query() {
            b'?'.hash(state);
            Hash::hash_slice(query.as_bytes(), state);
        }
    }