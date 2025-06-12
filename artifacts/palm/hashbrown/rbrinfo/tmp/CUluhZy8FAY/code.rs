pub unsafe fn erase(&mut self, item: Bucket<T>) {
        // Erase the element from the table first since drop might panic.
        self.erase_no_drop(&item);
        item.drop();
    }