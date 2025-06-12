fn new(threshold: u64, mut reseeder: Rsdr) -> Result<Self, Rsdr::Error> {
        // Because generating more values than `i64::MAX` takes centuries on
        // current hardware, we just clamp to that value.
        // Also we set a threshold of 0, which indicates no limit, to that
        // value.
        let threshold = if threshold == 0 {
            i64::MAX
        } else if threshold <= i64::MAX as u64 {
            threshold as i64
        } else {
            i64::MAX
        };

        let inner = R::try_from_rng(&mut reseeder)?;

        Ok(ReseedingCore {
            inner,
            reseeder,
            threshold,
            bytes_until_reseed: threshold,
        })
    }