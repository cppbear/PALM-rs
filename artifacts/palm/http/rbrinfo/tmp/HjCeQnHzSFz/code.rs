pub fn build(self) -> Result<Uri, crate::Error> {
        let parts = self.parts?;
        Uri::from_parts(parts).map_err(Into::into)
    }