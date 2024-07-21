use std::collections::{BTreeSet, HashMap, HashSet, LinkedList};
use dashmap::DashMap;

use crate::datatype::RedisCollection;
use crate::storage::Storage;

pub struct MemStorage {
    strings: HashMap<String, LinkedList<Vec<u8>>>,
    lists: HashMap<String, LinkedList<Vec<u8>>>,
    sets: HashMap<String, HashSet<Vec<u8>>>,
    sorted_sets: HashMap<String, BTreeSet<Vec<u8>>>,
    hashes: HashMap<String, HashMap<String, Vec<u8>>>,
}

impl MemStorage {

}

impl Storage for MemStorage {
    fn put(&self, key: String, value: RedisCollection) {
        todo!()
    }

    fn get(&self, key: &str) -> Option<RedisCollection> {
        todo!()
    }

    fn remove(&self, key: &str) {
        todo!()
    }
}
