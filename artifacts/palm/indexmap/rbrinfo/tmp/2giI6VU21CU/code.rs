fn erase_index(table: &mut Indices, hash: HashValue, index: usize) {
    if let Ok(entry) = table.find_entry(hash.get(), move |&i| i == index) {
        entry.remove();
    } else if cfg!(debug_assertions) {
        panic!("index not found");
    }
}