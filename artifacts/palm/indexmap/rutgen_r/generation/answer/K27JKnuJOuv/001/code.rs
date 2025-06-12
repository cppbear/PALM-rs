// Answer 0

#[derive(Default)]
struct TestMap<T> {
    data: Vec<T>,
}

impl<T> TestMap<T> {
    fn get_index_mut2(&mut self, index: usize) -> Option<(&mut T, ())> {
        if index < self.data.len() {
            Some((&mut self.data[index], ()))
        } else {
            None
        }
    }
}

struct TestStruct {
    map: TestMap<i32>,
}

impl TestStruct {
    fn get_index_mut2(&mut self, index: usize) -> Option<&mut i32> {
        match self.map.get_index_mut2(index) {
            Some((value, ())) => Some(value),
            None => None,
        }
    }
}

#[test]
fn test_get_index_mut2_valid_index() {
    let mut test_struct = TestStruct {
        map: TestMap {
            data: vec![1, 2, 3],
        },
    };
    let result = test_struct.get_index_mut2(1);
    assert_eq!(result, Some(&mut 2));
}

#[test]
fn test_get_index_mut2_invalid_index() {
    let mut test_struct = TestStruct {
        map: TestMap {
            data: vec![1, 2, 3],
        },
    };
    let result = test_struct.get_index_mut2(5);
    assert_eq!(result, None);
}

#[test]
fn test_get_index_mut2_boundary_index_lower() {
    let mut test_struct = TestStruct {
        map: TestMap {
            data: vec![10, 20, 30],
        },
    };
    let result = test_struct.get_index_mut2(0);
    assert_eq!(result, Some(&mut 10));
}

#[test]
fn test_get_index_mut2_boundary_index_upper() {
    let mut test_struct = TestStruct {
        map: TestMap {
            data: vec![10, 20, 30],
        },
    };
    let result = test_struct.get_index_mut2(2);
    assert_eq!(result, Some(&mut 30));
}

#[test]
#[should_panic]
fn test_get_index_mut2_panic() {
    let mut test_struct = TestStruct {
        map: TestMap {
            data: vec![1],
        },
    };
    // This index will trigger a panic since it's out of bounds.
    let _ = test_struct.get_index_mut2(1).unwrap();
}

