fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ips: Vec<usize> = self.inst_ptrs().collect();
        f.debug_struct("State")
         .field("flags", &self.flags())
         .field("insts", &ips)
         .finish()
    }