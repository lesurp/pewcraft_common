use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct Id<T: Eq>(usize, PhantomData<T>);

impl<T: Eq> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T: Eq> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Eq> Id<T> {
    fn new(u: usize) -> Self {
        Id(u, PhantomData)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Map<T: Eq>(HashMap<Id<T>, T>);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MapBuilder<T: Eq>(HashMap<Id<T>, T>, usize);

impl<T: Eq> MapBuilder<T> {
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
