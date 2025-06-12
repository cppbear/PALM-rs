pub fn union<'a>(&'a self, other: &'a Self) -> Union<'a, T, S, A> {
        // We'll iterate one set in full, and only the remaining difference from the other.
        // Use the smaller set for the difference in order to reduce hash lookups.
        let (smaller, larger) = if self.len() <= other.len() {
            (self, other)
        } else {
            (other, self)
        };
        Union {
            iter: larger.iter().chain(smaller.difference(larger)),
        }
    }