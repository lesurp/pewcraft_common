use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

#[derive(Serialize, Deserialize, Debug)]
#[serde(from = "usize", into = "usize")]
pub struct Id<T>(usize, PhantomData<T>);

impl<T> From<usize> for Id<T> {
    fn from(id: usize) -> Self {
        Id::new(id)
    }
}

impl<T> From<Id<T>> for usize {
    fn from(id: Id<T>) -> Self {
        id.0
    }
}

impl<T> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

/// Copy needs to be defined manually because of the PhantomData marker
/// We can derive Copy but it simply doesn't work (no compilation error)
impl<T> Copy for Id<T> {}

/// Because we define Copy manually, we need to also define Clone manually
impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T> Eq for Id<T> {}

impl<T> Id<T> {
    pub fn new(id: usize) -> Self {
        Id(id, PhantomData)
    }

    pub fn raw(self) -> usize {
        self.0
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Map<T>(HashMap<Id<T>, T>);

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MapBuilder<T>(HashMap<Id<T>, T>, usize);

impl<T> MapBuilder<T> {
    pub fn new() -> Self {
        MapBuilder(HashMap::new(), 0)
    }

    pub fn add(&mut self, t: T) -> Id<T> {
        let id = Id::new(self.1);
        self.0.insert(id, t);
        self.1 += 1;
        id
    }

    pub fn build(self) -> Map<T> {
        Map(self.0)
    }
}
