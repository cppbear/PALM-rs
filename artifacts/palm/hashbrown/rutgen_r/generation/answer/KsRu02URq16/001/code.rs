// Answer 0

#[derive(Debug, PartialEq, Eq, Hash)]
struct Key;

#[derive(Debug)]
struct Value;

#[derive(Default)]
struct MyMap {
    table: Vec<(Key, Value)>,
}

impl MyMap {
    fn build_hashes_inner<Q, const N: usize>(&self, ks: [&Q; N]) -> Vec<usize>
    where
        Q: Hash + Equivalent<Key> + ?Sized,
    {
        ks.iter().map(|k| self.calculate_hash(k)).collect()
    }

    fn calculate_hash<Q>(&self, key: &Q) -> usize
    where
        Q: Hash + Equivalent<Key> + ?Sized,
    {
        // Simulate a hash calculation using the address of the key for simplicity
        std::mem::discriminant(&key) as usize
    }
    
    fn get_many_mut<Q, const N: usize>(&mut self, hashes: Vec<usize>, equivalent_fn: impl Fn(usize, &(Key, Value))) -> [Option<&'_ mut (Key, Value)>; N] {
        let mut result = Default::default();
        for (i, hash) in hashes.iter().enumerate() {
            result[i] = self.table.iter_mut().find(|(k, _)| equivalent_fn(*hash, (k, &self.table[*hash]))) 
        }
        result
    }
    
    fn get_many_mut_inner<Q, const N: usize>(&mut self, ks: [&Q; N]) -> [Option<&'_ mut (Key, Value)>; N]
    where
        Q: Hash + Equivalent<Key> + ?Sized,
    {
        let hashes = self.build_hashes_inner(ks);
        self.get_many_mut(hashes, |i, (k, _)| ks[i].equivalent(k))
    }
}

trait Equivalent<T> {
    fn equivalent(&self, other: &T) -> bool;
}

impl Equivalent<Key> for Key {
    fn equivalent(&self, _other: &Key) -> bool {
        true
    }
}

#[test]
fn test_get_many_mut_inner_with_valid_keys() {
    let mut my_map = MyMap::default();
    my_map.table.push((Key, Value));
    my_map.table.push((Key, Value));

    let keys = [&Key; 2];
    let result = my_map.get_many_mut_inner(keys);
    
    assert!(result[0].is_some());
    assert!(result[1].is_some());
}

#[test]
fn test_get_many_mut_inner_with_empty_map() {
    let mut my_map = MyMap::default();

    let keys = [&Key; 2];
    let result = my_map.get_many_mut_inner(keys);
    
    assert!(result[0].is_none());
    assert!(result[1].is_none());
}

#[test]
#[should_panic]
fn test_get_many_mut_inner_panics_on_out_of_bounds() {
    let mut my_map = MyMap::default();
    my_map.table.push((Key, Value));

    let keys = [&Key; 3]; // One less element in the map
    my_map.get_many_mut_inner(keys);
}

