pub fn iter(&self) -> SubCapturesPosIter {
        SubCapturesPosIter { idx: 0, locs: self }
    }