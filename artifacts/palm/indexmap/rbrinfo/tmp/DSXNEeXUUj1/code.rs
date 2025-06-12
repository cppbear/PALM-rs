pub(crate) const fn new() -> Self {
        IndexMapCore {
            indices: Indices::new(),
            entries: Vec::new(),
        }
    }