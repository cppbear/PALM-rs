fn is_in_same_group(&self, i: usize, new_i: usize, hash: u64) -> bool {
        let probe_seq_pos = self.probe_seq(hash).pos;
        let probe_index =
            |pos: usize| (pos.wrapping_sub(probe_seq_pos) & self.bucket_mask) / Group::WIDTH;
        probe_index(i) == probe_index(new_i)
    }