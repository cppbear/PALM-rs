// Answer 0

#[derive(Debug)]
struct TestMap {
    keys: Vec<Field>,
    values: Vec<Idx>,
    index: usize,
}

impl TestMap {
    fn new(keys: Vec<Field>, values: Vec<Idx>) -> Self {
        Self { keys, values, index: 0 }
    }
}

impl<'de> MapAccess<'de> for TestMap {
    type Error = &'static str;

    fn next_key(&mut self) -> Result<Option<Field>, Self::Error> {
        if self.index < self.keys.len() {
            let key = self.keys[self.index].clone();
            self.index += 1;
            Ok(Some(key))
        } else {
            Ok(None)
        }
    }

    fn next_value<T>(&mut self) -> Result<T, Self::Error>
    where
        T: Deserialize<'de>,
    {
        Err("next_value error")
    }
}

#[test]
fn test_visit_map_with_error_on_next_value() {
    let keys = vec![Field::End];
    let values: Vec<Idx> = vec![];
    let map = TestMap::new(keys, values);

    let result: Result<Idx, &str> = visit_map(map);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "next_value error");
}

#[derive(Clone)]
enum Field {
    End,
}

struct Idx;

fn visit_map<A>(mut map: A) -> Result<Idx, A::Error>
where
    A: MapAccess<'de>,
{
    let mut end: Option<Idx> = None;
    while let Some(key) = tri!(map.next_key()) {
        match key {
            Field::End => {
                if end.is_some() {
                    return Err(<A::Error as Error>::duplicate_field("end"));
                }
                end = Some(tri!(map.next_value()));
            }
        }
    }
    let end = match end {
        Some(end) => end,
        None => return Err(<A::Error as Error>::missing_field("end")),
    };
    Ok(end)
}

trait MapAccess<'de> {
    type Error;

    fn next_key(&mut self) -> Result<Option<Field>, Self::Error>;
    fn next_value<T>(&mut self) -> Result<T, Self::Error>
    where
        T: Deserialize<'de>;
}

trait Error {
    fn duplicate_field(field: &str) -> Self;
    fn missing_field(field: &str) -> Self;
}

