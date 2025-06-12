// Answer 0

#[derive(Debug)]
struct GetDisjointMutError;

struct MyMap<K, V> {
    data: Vec<(K, V)>,
}

impl<K, V> MyMap<K, V> {
    fn new() -> Self {
        MyMap { data: Vec::new() }
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn get_disjoint_opt_mut<const N: usize>(
        &mut self,
        indices: [Option<usize>; N],
    ) -> Result<[Option<(&K, &mut V)>; N], GetDisjointMutError> {
        let mut result = [None; N];
        for (i, &index) in indices.iter().enumerate() {
            if let Some(index) = index {
                if index < self.len() && result.iter().all(|item| item.is_none() || item.unwrap().0 != &self.data[index].0) {
                    let pair = &mut self.data[index];
                    result[i] = Some((&pair.0, &mut pair.1));
                } else {
                    return Err(GetDisjointMutError);
                }
            }
        }
        Ok(result)
    }

    pub fn get_disjoint_mut<const N: usize>(
        &mut self,
        indices: [usize; N],
    ) -> Result<[(&K, &mut V); N], GetDisjointMutError> {
        let indices = indices.map(Some);
        let key_values = self.get_disjoint_opt_mut(indices)?;
        Ok(key_values.map(Option::unwrap))
    }
}

#[test]
fn test_get_disjoint_mut_empty() {
    let mut my_map: MyMap<i32, i32> = MyMap::new();
    let result: Result<[(&i32, &mut i32); 0], GetDisjointMutError> = my_map.get_disjoint_mut([]);
    assert!(result.is_ok());
}

#[test]
fn test_get_disjoint_mut_single_index() {
    let mut my_map = MyMap::new();
    my_map.data.push((1, 100));
    let result = my_map.get_disjoint_mut([0]);
    assert!(result.is_ok());
    let (key, value) = result.unwrap()[0];
    assert_eq!(*key, 1);
    *value += 50; 
    assert_eq!(my_map.data[0].1, 150);
}

#[test]
fn test_get_disjoint_mut_duplicates() {
    let mut my_map = MyMap::new();
    my_map.data.push((1, 100));
    my_map.data.push((2, 200));
    my_map.data.push((3, 300));
    let result = my_map.get_disjoint_mut([0, 0]);
    assert!(result.is_err());
}

#[test]
fn test_get_disjoint_mut_out_of_bounds() {
    let mut my_map = MyMap::new();
    my_map.data.push((1, 100));
    my_map.data.push((2, 200));
    let result = my_map.get_disjoint_mut([0, 2]);
    assert!(result.is_err());
}

#[test]
fn test_get_disjoint_mut_non_unique() {
    let mut my_map = MyMap::new();
    my_map.data.push((1, 100));
    my_map.data.push((2, 200));
    let result = my_map.get_disjoint_mut([1, 1]);
    assert!(result.is_err());
}

#[test]
fn test_get_disjoint_mut_multiple_valid() {
    let mut my_map = MyMap::new();
    my_map.data.push((1, 100));
    my_map.data.push((2, 200));
    my_map.data.push((3, 300));
    let result = my_map.get_disjoint_mut([0, 1, 2]);
    assert!(result.is_ok());
    let values = result.unwrap();
    assert_eq!(*values[0].0, 1);
    assert_eq!(*values[1].0, 2);
    assert_eq!(*values[2].0, 3);
}

