pub fn extract_if<F>(&mut self, f: F) -> ExtractIf<'_, T, F, A>
    where
        F: FnMut(&T) -> bool,
    {
        ExtractIf {
            f,
            inner: RawExtractIf {
                iter: unsafe { self.map.table.iter() },
                table: &mut self.map.table,
            },
        }
    }