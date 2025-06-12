pub fn weight(&self, index: usize) -> Option<X>
    where
        X: for<'a> core::ops::SubAssign<&'a X>,
    {
        use core::cmp::Ordering::*;

        let mut weight = match index.cmp(&self.cumulative_weights.len()) {
            Less => self.cumulative_weights[index].clone(),
            Equal => self.total_weight.clone(),
            Greater => return None,
        };

        if index > 0 {
            weight -= &self.cumulative_weights[index - 1];
        }
        Some(weight)
    }