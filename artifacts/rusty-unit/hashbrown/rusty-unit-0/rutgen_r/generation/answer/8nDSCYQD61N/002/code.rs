// Answer 0

#[test]
fn test_and_modify_occupied_entry() {
    use hashbrown::HashMap;
    
    struct EntryRef<V> {
        occupied: Option<V>,
    }
    
    impl<V> EntryRef<V> {
        pub fn get_mut(&mut self) -> &mut V {
            self.occupied.as_mut().expect("Entry should be occupied")
        }
        
        pub fn and_modify<F>(self, f: F) -> Self
        where
            F: FnOnce(&mut V),
        {
            match self.occupied {
                Some(ref mut entry) => {
                    f(entry);
                    EntryRef { occupied: Some(entry.clone()) }
                }
                None => EntryRef { occupied: None },
            }
        }
    }
    
    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert("poneyland".to_string(), 42);
    
    let mut entry_ref = EntryRef {
        occupied: map.get_mut("poneyland"),
    };
    
    entry_ref.and_modify(|e| *e += 1);
    
    assert_eq!(map["poneyland"], 43);
}

#[test]
#[should_panic(expected = "Entry should be occupied")]
fn test_and_modify_vacant_entry() {
    use hashbrown::HashMap;
    
    struct EntryRef<V> {
        occupied: Option<V>,
    }
    
    impl<V> EntryRef<V> {
        pub fn get_mut(&mut self) -> &mut V {
            self.occupied.as_mut().expect("Entry should be occupied")
        }
        
        pub fn and_modify<F>(self, f: F) -> Self
        where
            F: FnOnce(&mut V),
        {
            match self.occupied {
                Some(ref mut entry) => {
                    f(entry);
                    EntryRef { occupied: Some(entry.clone()) }
                }
                None => EntryRef { occupied: None },
            }
        }
    }
    
    let mut map: HashMap<String, u32> = HashMap::new();
    
    let mut entry_ref = EntryRef {
        occupied: map.get_mut("nonexistent"),
    };
    
    entry_ref.and_modify(|e| *e += 1);
}

