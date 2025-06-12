pub(crate) fn match_empty(self) -> BitMask {
        self.match_tag(Tag::EMPTY)
    }