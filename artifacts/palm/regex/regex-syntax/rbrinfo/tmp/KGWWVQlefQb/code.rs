fn symmetric_difference(
        &self,
        other: &Self,
    ) -> (Option<Self>, Option<Self>) {
        let union = match self.union(other) {
            None => return (Some(self.clone()), Some(other.clone())),
            Some(union) => union,
        };
        let intersection = match self.intersect(other) {
            None => return (Some(self.clone()), Some(other.clone())),
            Some(intersection) => intersection,
        };
        union.difference(&intersection)
    }