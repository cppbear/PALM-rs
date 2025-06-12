pub(super) fn is_none(&self) -> bool {
        matches!(*self, Scheme2::None)
    }