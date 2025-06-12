fn find_iter (
        self,
        text: &Self::Text,
    ) -> Matches<Self> {
        Matches {
            re: self,
            text: text,
            last_end: 0,
            last_match: None,
        }
    }