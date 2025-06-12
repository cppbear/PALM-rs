pub fn is_i64(&self) -> bool {
        #[cfg(not(feature = "arbitrary_precision"))]
        match self.n {
            N::PosInt(v) => v <= i64::MAX as u64,
            N::NegInt(_) => true,
            N::Float(_) => false,
        }
        #[cfg(feature = "arbitrary_precision")]
        self.as_i64().is_some()
    }