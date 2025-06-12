pub fn extract_if<F>(&mut self, f: F) -> ExtractIf<'_, K, V, F, A>
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        ExtractIf {
            f,
            inner: RawExtractIf {
                iter: unsafe { self.table.iter() },
                table: &mut self.table,
            },
        }
    }