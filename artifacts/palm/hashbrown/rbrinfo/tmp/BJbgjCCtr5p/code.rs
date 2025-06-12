pub fn symmetric_difference<'a>(&'a self, other: &'a Self) -> SymmetricDifference<'a, T, S, A> {
        SymmetricDifference {
            iter: self.difference(other).chain(other.difference(self)),
        }
    }