fn update_index(table: &mut Indices, hash: HashValue, old: usize, new: usize) {
    let index = table
        .find_mut(hash.get(), move |&i| i == old)
        .expect("index not found");
    *index = new;
}