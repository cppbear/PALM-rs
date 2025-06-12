fn drop(&mut self) {
        unsafe {
            // Drop all remaining elements. Note that this may panic.
            self.iter.drop_elements();

            // Reset the contents of the table now that all elements have been
            // dropped.
            self.table.clear_no_drop();

            // Move the now empty table back to its original location.
            self.orig_table
                .as_ptr()
                .copy_from_nonoverlapping(&self.table, 1);
        }
    }