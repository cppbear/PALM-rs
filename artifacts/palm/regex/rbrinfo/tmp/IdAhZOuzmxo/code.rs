fn prefix_at(
        &self,
        prefixes: &LiteralSearcher,
        at: InputAt,
    ) -> Option<InputAt> {
        prefixes.find(&self[at.pos()..]).map(|(s, _)| self.at(at.pos() + s))
    }