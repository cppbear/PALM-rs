fn is_subset(&self, other: &Self) -> bool {
        let (lower1, upper1) = (self.lower(), self.upper());
        let (lower2, upper2) = (other.lower(), other.upper());
        (lower2 <= lower1 && lower1 <= upper2)
        && (lower2 <= upper1 && upper1 <= upper2)
    }