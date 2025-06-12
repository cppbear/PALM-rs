pub(crate) fn with_capacity(n: usize) -> Self {
        IndexMapCore {
            indices: Indices::with_capacity(n),
            entries: Vec::with_capacity(n),
        }
    }