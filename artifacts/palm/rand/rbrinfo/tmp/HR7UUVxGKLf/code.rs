pub fn weights(&self) -> WeightedIndexIter<'_, X>
    where
        X: for<'a> core::ops::SubAssign<&'a X>,
    {
        WeightedIndexIter {
            weighted_index: self,
            index: 0,
        }
    }