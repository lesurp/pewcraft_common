use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Id<T>(usize, PhantomData<T>);

impl<T> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T> Eq for Id<T> {}

impl<T> Id<T> {
    pub fn new(u: usize) -> Self {
        Id(u, PhantomData)
    }

    pub fn raw(self) -> usize {
        self.0
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Map<T>(HashMap<Id<T>, T>);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MapBuilder<T>(HashMap<Id<T>, T>, usize);

impl<T> MapBuilder<T> {
    pub fn new() -> Self {
        MapBuilder(HashMap::new(), 0)
    }

    pub fn add(&mut self, t: T) {
        self.0.insert(Id::new(self.1), t);
        self.1 += 1;
    }

    pub fn build(self) -> Map<T> {
        Map(self.0)
    }
}
