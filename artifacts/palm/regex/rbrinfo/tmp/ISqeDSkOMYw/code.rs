fn prefix_at(
        &self,
        prefixes: &LiteralSearcher,
        at: InputAt,
    ) -> Option<InputAt> {
        (**self).prefix_at(prefixes, at)
    }