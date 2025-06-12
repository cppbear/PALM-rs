pub(crate) fn match_full(&self) -> BitMask {
        self.match_empty_or_deleted().invert()
    }