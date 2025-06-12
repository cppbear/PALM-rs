pub fn difference<'a>(&'a self, other: &'a Self) -> Difference<'a, T, S, A> {
        Difference {
            iter: self.iter(),
            other,
        }
    }