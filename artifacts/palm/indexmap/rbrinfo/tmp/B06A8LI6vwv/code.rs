pub fn get_disjoint_mut<const N: usize>(
        &mut self,
        indices: [usize; N],
    ) -> Result<[(&K, &mut V); N], GetDisjointMutError> {
        let indices = indices.map(Some);
        let key_values = self.get_disjoint_opt_mut(indices)?;
        Ok(key_values.map(Option::unwrap))
    }