fn try_copy_to_slice(&mut self, dst: &mut [u8]) -> Result<(), TryGetError> {
            (**self).try_copy_to_slice(dst)
        }