use std::collections::{HashMap};
use std::hash::Hash;

// Custom map which tracks insertion order of entries
#[derive(Debug)]
pub struct Map<K,V> 
    where K: Eq + Hash + Copy
{
 
    keys: Vec<K>,
    map: HashMap::<K, V>
    
}

impl <K,V> Map<K,V> 
    where K: Eq + Hash + Copy
{
    
    pub fn new() -> Self{
        Map {
            keys: vec![],
            map: HashMap::<K,V>::new(),
        }
    }
    
    pub fn insert(&mut self, key: K, val:V) {
        if self.index_of(&key).is_none() {
            self.keys.push(key);
        }
        self.map.insert(key, val);
    }

    pub fn remove(&mut self, key: K) {
        match self.index_of(&key) {
            Some(pos) => {
                self.keys.remove(pos);
                self.map.remove(&key);
            },
            _ => ()
        }
        
    }

    pub fn keys(&self) -> &Vec<K> {
        &self.keys
    }

    pub fn get_entry(&self, key: &K) -> Option<(&K, &V)> {
        self.map.get_key_value(key)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    pub fn index_of(&self, key: &K) -> Option<usize> {
        self.keys.iter()
            .position(|e| e == key)
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }
    
}