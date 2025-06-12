fn next_u64(&mut self) -> u64 {
        let res = output_dxsm(self.state);
        self.step();
        res
    }