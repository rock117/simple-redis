use crate::storage::Storage;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct SortedSets(BTreeSet<Item>);

#[derive(Clone, Debug, Eq)]
struct Item {
    value: String,
    score: Float,
}

#[derive(Copy, Clone, Debug)]
struct Float(f64);

impl SortedSets {
    pub fn zadd(&mut self, item: Item) {
        if self.0.contains(&item) {
            self.0.remove(&item);
        }
        self.0.insert(item);
    }
}

impl PartialEq<Self> for Item {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialOrd<Self> for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl Hash for Item {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }
}

impl Float {
    pub fn new_f64(f: f64) -> Self {
        Self(f)
    }
    pub fn new_f32(f: f32) -> Self {
        Self(f as f64)
    }
    pub fn new_usize(f: usize) -> Self {
        Self(f as f64)
    }
}

impl Eq for Float {}

impl PartialEq<Self> for Float {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd<Self> for Float {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Float {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}
