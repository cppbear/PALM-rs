fn is_intersection_empty(&self, other: &Self) -> bool {
        let (lower1, upper1) = (self.lower(), self.upper());
        let (lower2, upper2) = (other.lower(), other.upper());
        cmp::max(lower1, lower2) > cmp::min(upper1, upper2)
    }