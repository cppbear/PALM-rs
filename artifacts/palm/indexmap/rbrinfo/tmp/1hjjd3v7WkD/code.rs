pub fn new_mut<'a>() -> &'a mut Self {
        Self::from_mut_slice(&mut [])
    }