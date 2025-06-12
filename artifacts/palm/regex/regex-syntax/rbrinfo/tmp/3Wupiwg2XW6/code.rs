pub fn is_negated(&self) -> bool {
        match self.kind {
            ClassUnicodeKind::NamedValue {
                op: ClassUnicodeOpKind::NotEqual, ..
            } => !self.negated,
            _ => self.negated,
        }
    }