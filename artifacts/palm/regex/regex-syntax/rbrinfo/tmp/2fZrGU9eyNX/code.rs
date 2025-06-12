pub fn auxiliary_span(&self) -> Option<&Span> {
        use self::ErrorKind::*;
        match self.kind {
            FlagDuplicate { ref original } => Some(original),
            FlagRepeatedNegation { ref original, .. } => Some(original),
            GroupNameDuplicate { ref original, .. } => Some(original),
            _ => None,
        }
    }