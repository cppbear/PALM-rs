pub fn is_capturing(&self) -> bool {
        match self.kind {
            GroupKind::CaptureIndex(_) | GroupKind::CaptureName(_) => true,
            GroupKind::NonCapturing(_) => false,
        }
    }