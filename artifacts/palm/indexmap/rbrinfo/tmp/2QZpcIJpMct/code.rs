pub(super) fn new<R>(set: &'a mut IndexSet<T, S>, range: R, replace_with: I) -> Self
    where
        R: RangeBounds<usize>,
    {
        Self {
            iter: set.map.splice(range, UnitValue(replace_with)),
        }
    }