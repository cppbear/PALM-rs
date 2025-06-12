pub fn is_match_empty(&self) -> bool {
        match self.kind {
            RepetitionKind::ZeroOrOne => true,
            RepetitionKind::ZeroOrMore => true,
            RepetitionKind::OneOrMore => false,
            RepetitionKind::Range(RepetitionRange::Exactly(m)) => m == 0,
            RepetitionKind::Range(RepetitionRange::AtLeast(m)) => m == 0,
            RepetitionKind::Range(RepetitionRange::Bounded(m, _)) => m == 0,
        }
    }