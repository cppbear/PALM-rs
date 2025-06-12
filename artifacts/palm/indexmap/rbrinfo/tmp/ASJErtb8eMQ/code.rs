pub(crate) fn get_disjoint_opt_mut<const N: usize>(
        &mut self,
        indices: [Option<usize>; N],
    ) -> Result<[Option<(&K, &mut V)>; N], GetDisjointMutError> {
        // SAFETY: Can't allow duplicate indices as we would return several mutable refs to the same data.
        let len = self.len();
        for i in 0..N {
            if let Some(idx) = indices[i] {
                if idx >= len {
                    return Err(GetDisjointMutError::IndexOutOfBounds);
                } else if indices[..i].contains(&Some(idx)) {
                    return Err(GetDisjointMutError::OverlappingIndices);
                }
            }
        }

        let entries_ptr = self.entries.as_mut_ptr();
        let out = indices.map(|idx_opt| {
            match idx_opt {
                Some(idx) => {
                    // SAFETY: The base pointer is valid as it comes from a slice and the reference is always
                    // in-bounds & unique as we've already checked the indices above.
                    let kv = unsafe { (*(entries_ptr.add(idx))).ref_mut() };
                    Some(kv)
                }
                None => None,
            }
        });

        Ok(out)
    }