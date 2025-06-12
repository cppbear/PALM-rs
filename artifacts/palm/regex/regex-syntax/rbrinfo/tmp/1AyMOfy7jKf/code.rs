fn create(lower: Self::Bound, upper: Self::Bound) -> Self {
        let mut int = Self::default();
        if lower <= upper {
            int.set_lower(lower);
            int.set_upper(upper);
        } else {
            int.set_lower(upper);
            int.set_upper(lower);
        }
        int
    }