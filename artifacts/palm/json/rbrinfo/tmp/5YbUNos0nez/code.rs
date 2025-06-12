pub fn as_i128(&self) -> Option<i128> {
        #[cfg(not(feature = "arbitrary_precision"))]
        match self.n {
            N::PosInt(n) => Some(n as i128),
            N::NegInt(n) => Some(n as i128),
            N::Float(_) => None,
        }
        #[cfg(feature = "arbitrary_precision")]
        self.n.parse().ok()
    }