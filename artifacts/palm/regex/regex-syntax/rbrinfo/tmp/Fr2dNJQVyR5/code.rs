pub fn capture_index(&self) -> Option<u32> {
        match self.kind {
            GroupKind::CaptureIndex(i) => Some(i),
            GroupKind::CaptureName(ref x) => Some(x.index),
            GroupKind::NonCapturing(_) => None,
        }
    }