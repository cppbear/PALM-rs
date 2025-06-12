pub(super) fn new<R>(map: &'a mut IndexMap<K, V, S>, range: R, replace_with: I) -> Self
    where
        R: RangeBounds<usize>,
    {
        let (tail, drain) = map.core.split_splice(range);
        Self {
            map,
            tail,
            drain,
            replace_with,
        }
    }