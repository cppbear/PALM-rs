pub fn refill4(&mut self, drounds: u32, out: &mut [u32; BUFSZ]) {
        refill_wide(self, drounds, out)
    }