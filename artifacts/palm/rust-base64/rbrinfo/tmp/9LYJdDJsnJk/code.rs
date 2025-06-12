fn drop(&mut self) {
        if !self.panicked {
            // like `BufWriter`, ignore errors during drop
            let _ = self.write_final_leftovers();
        }
    }