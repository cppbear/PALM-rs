pub fn extract_if<F>(&mut self, f: F) -> ExtractIf<'_, T, F, A>
    where
        F: FnMut(&mut T) -> bool,
    {
        ExtractIf {
            f,
            inner: RawExtractIf {
                iter: unsafe { self.raw.iter() },
                table: &mut self.raw,
            },
        }
    }