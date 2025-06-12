// Answer 0

#[derive(Debug)]
struct MyStruct {
    value: i32,
}

#[derive(Debug)]
struct MyKey {
    key: i32,
}

impl std::hash::Hash for MyKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

trait Equivalent<T> {
    fn equivalent(&self, other: &T) -> bool;
}

impl Equivalent<MyStruct> for MyKey {
    fn equivalent(&self, other: &MyStruct) -> bool {
        self.key == other.value
    }
}

struct MyMap {
    data: Vec<(MyKey, MyStruct)>,
}

impl MyMap {
    fn get_full_mut2<Q>(&mut self, value: &Q) -> Option<(usize, &mut MyStruct, ())>
    where
        Q: ?Sized + Hash + Equivalent<MyStruct>,
    {
        for (index, (key, struct_value)) in self.data.iter_mut().enumerate() {
            if value.equivalent(struct_value) {
                return Some((index, struct_value, ()));
            }
        }
        None
    }
}

struct Container {
    map: MyMap,
}

impl Container {
    fn new() -> Container {
        Container {
            map: MyMap {
                data: vec![
                    (MyKey { key: 1 }, MyStruct { value: 1 }),
                    (MyKey { key: 2 }, MyStruct { value: 2 }),
                ],
            },
        }
    }

    fn get_full_mut2<Q>(&mut self, value: &Q) -> Option<(usize, &mut MyStruct)>
    where
        Q: ?Sized + Hash + Equivalent<MyStruct>,
    {
        match self.map.get_full_mut2(value) {
            Some((index, value, ())) => Some((index, value)),
            None => None,
        }
    }
}

#[test]
fn test_get_full_mut2_found() {
    let mut container = Container::new();
    let key = MyKey { key: 1 };
    let result = container.get_full_mut2(&key);
    assert_eq!(result, Some((0, &mut MyStruct { value: 1 })));
}

#[test]
fn test_get_full_mut2_not_found() {
    let mut container = Container::new();
    let key = MyKey { key: 3 };
    let result = container.get_full_mut2(&key);
    assert_eq!(result, None);
}

