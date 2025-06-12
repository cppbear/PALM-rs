pub fn intersection<'a>(&'a self, other: &'a Self) -> Intersection<'a, T, S, A> {
        let (smaller, larger) = if self.len() <= other.len() {
            (self, other)
        } else {
            (other, self)
        };
        Intersection {
            iter: smaller.iter(),
            other: larger,
        }
    }