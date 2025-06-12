fn drop(&mut self) {
        unsafe {
            // Drop all remaining elements
            self.iter.drop_elements();

            // Free the table
            if let Some((ptr, layout, ref alloc)) = self.allocation {
                alloc.deallocate(ptr, layout);
            }
        }
    }