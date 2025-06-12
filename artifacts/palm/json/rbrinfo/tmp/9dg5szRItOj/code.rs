pub fn from_i128(i: i128) -> Option<Number> {
        let n = {
            #[cfg(not(feature = "arbitrary_precision"))]
            {
                if let Ok(u) = u64::try_from(i) {
                    N::PosInt(u)
                } else if let Ok(i) = i64::try_from(i) {
                    N::NegInt(i)
                } else {
                    return None;
                }
            }
            #[cfg(feature = "arbitrary_precision")]
            {
                i.to_string()
            }
        };
        Some(Number { n })
    }