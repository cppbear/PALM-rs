fn random_seed() -> Option<u64> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::thread;
    use std::time::Instant;

    let mut hasher = DefaultHasher::new();
    Instant::now().hash(&mut hasher);
    thread::current().id().hash(&mut hasher);
    Some(hasher.finish())
}