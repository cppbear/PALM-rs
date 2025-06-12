// Answer 0

#[derive(Default)]
struct ValueType {
    value: i32,
}

struct Element {
    key: u32,
    value: ValueType,
}

struct Container {
    map: Option<Vec<Element>>,
    fill: usize,
    mask: usize,
    used: usize,
}

impl Container {
    fn new(mask: usize) -> Self {
        Self {
            map: Some(vec![Element::default(); mask + 1]),
            fill: 0,
            mask,
            used: 0,
        }
    }

    fn allocate(&mut self) {
        // initialization logic can be defined or more elements can be added as needed
        self.map = Some(vec![Element::default(); self.mask + 1]);
    }

    fn lookup(&self, key: u32) -> usize {
        // simplistic lookup logic for test cases, for demonstration purposes
        (key as usize) & self.mask
    }

    fn grow(&mut self, new_size: usize) {
        // grow logic, for the purpose of tests can be a no-op
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
            // resize when 2/3 full
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
fn test_get_mut_initial_empty() {
    let mut container = Container::new(3);  // mask set to 3 which means we have a table of size 4
    let value = container.get_mut(1); 
    assert_eq!(value.value, 0);
}

#[test]
fn test_get_mut_existing_value() {
    let mut container = Container::new(3);
    {
        let _value = container.get_mut(2);
        _value.value = 10; // set a value
    }
    let value = container.get_mut(2);
    assert_eq!(value.value, 10);
}

#[test]
fn test_get_mut_resizing() {
    let mut container = Container::new(2); // mask set to 2, allows for 3 fills before resize
    for key in 0..3 {
        let _ = container.get_mut(key);
    }

    let value = container.get_mut(4); // should cause a resize
    assert_eq!(value.value, 0);
} 

#[test]
#[should_panic(expected = "map should have been created above")]
fn test_get_mut_map_none() {
    let mut container = Container {
        map: None,
        fill: 0,
        mask: 3,
        used: 0,
    };
    container.get_mut(1);
}

