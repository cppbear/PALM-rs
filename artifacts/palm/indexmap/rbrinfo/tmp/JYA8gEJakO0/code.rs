pub fn get_disjoint_mut<Q, const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N]
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {
        let indices = keys.map(|key| self.get_index_of(key));
        match self.as_mut_slice().get_disjoint_opt_mut(indices) {
            Err(GetDisjointMutError::IndexOutOfBounds) => {
                unreachable!(
                    "Internal error: indices should never be OOB as we got them from get_index_of"
                );
            }
            Err(GetDisjointMutError::OverlappingIndices) => {
                panic!("duplicate keys found");
            }
            Ok(key_values) => key_values.map(|kv_opt| kv_opt.map(|kv| kv.1)),
        }
    }