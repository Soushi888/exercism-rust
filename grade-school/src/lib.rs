use std::collections::{BTreeMap, BTreeSet};

#[allow(clippy::new_without_default)]
pub struct School(BTreeMap<u32, BTreeSet<String>>);

impl School {
    pub fn new() -> School {
        School(BTreeMap::new())
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.0.entry(grade).or_insert(BTreeSet::new()).insert(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.0.keys().cloned().collect()
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.0.get(&grade).map_or_else(Vec::new, |s| s.iter().cloned().collect())
    }
}
