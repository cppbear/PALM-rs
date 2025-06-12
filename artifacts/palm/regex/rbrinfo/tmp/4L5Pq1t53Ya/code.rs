pub fn capture_names(&self) -> CaptureNames {
        CaptureNames(self.0.capture_names().iter())
    }