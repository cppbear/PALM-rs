// Answer 0

#[derive(Default)]
struct ValueType {
    value: i32,
}

struct MyMap {
    map: Option<Vec<MyEntry>>,
    fill: usize,
    used: usize,
    mask: usize,
}

#[derive(Default)]
struct MyEntry {
    key: u32,
    value: ValueType,
}

impl MyMap {
    fn new(mask: usize) -> Self {
        MyMap {
            map: None,
            fill: 0,
            used: 0,
            mask,
        }
    }

    fn allocate(&mut self) {
        self.map = Some(vec![MyEntry::default(); self.mask + 1]);
    }

    fn lookup(&self, key: u32) -> usize {
        (key as usize) & self.mask
    }

    fn grow(&mut self, new_size: usize) {
        // Simplified grow logic for testing purposes
        self.mask = new_size - 1;
    }

    fn get_mut(&mut self, key: u32) -> &mut ValueType {
        if self.map.is_none() {
            self.allocate();
        }

        let mut i = self.lookup(key);
        if self
            .map
            .as_ref()
            .expect("map should have been created above")[i]
            .value
            == Default::default()
        {
            self.fill += 1;
            if self.fill * 3 >= (self.mask + 1) * 2 {
                self.grow((self.used + 1) * 2);
                i = self.lookup(key);
            }

            self.used += 1;
        }

        let elem = &mut self
            .map
            .as_mut()
            .expect("map should have been created above")[i];
        elem.key = key;
        &mut elem.value
    }
}

#[test]
fn test_get_mut_initial_insertion() {
    let mut my_map = MyMap::new(3);
    let value_ref = my_map.get_mut(1);
    value_ref.value = 42;
    assert_eq!(my_map.map.as_ref().unwrap()[my_map.lookup(1)].value.value, 42);
}

#[test]
fn test_get_mut_existing_key_update() {
    let mut my_map = MyMap::new(3);
    let value_ref1 = my_map.get_mut(1);
    value_ref1.value = 42;
    
    let value_ref2 = my_map.get_mut(1);
    value_ref2.value = 100;

    assert_eq!(my_map.map.as_ref().unwrap()[my_map.lookup(1)].value.value, 100);
}

#[test]
fn test_get_mut_resize_on_fill() {
    let mut my_map = MyMap::new(2); // mask is 2, size is 3
    for i in 0..3 {
        my_map.get_mut(i);
    }
    let value_ref = my_map.get_mut(3);
    value_ref.value = 56;
    assert_eq!(my_map.map.as_ref().unwrap().len(), 8); // Check if resized correctly
}

