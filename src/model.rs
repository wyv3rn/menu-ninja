use std::{
    collections::{BTreeMap, btree_map::Entry},
    time::{SystemTime, UNIX_EPOCH},
};

use bjw_db_derive::derive_bjw_db;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Dish {
    name: String,
    last_cooked: Option<u64>,
}

impl Dish {
    pub fn new(name: &str) -> Self {
        Dish {
            name: name.to_string(),
            last_cooked: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_last_cooked(&mut self, time: u64) {
        self.last_cooked = Some(time);
    }

    pub fn last_cooked(&self) -> Option<u64> {
        self.last_cooked
    }

    pub fn not_cooked_for(&self) -> Option<u64> {
        self.last_cooked.map(|t| now() - t)
    }
}

pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[derive(Default, Serialize, Deserialize, Clone)]
struct Dishes {
    dishes: BTreeMap<String, Dish>,
}

#[derive_bjw_db(thread_safe)]
impl Dishes {
    pub fn new_dish(&mut self, name: String) {
        // TODO dependency: return a failure indication if entry exists
        if let Entry::Vacant(e) = self.dishes.entry(name.clone()) {
            let dish = Dish::new(&name);
            e.insert(dish);
        }
    }

    pub fn get_dish(&self, name: &str) -> Option<Dish> {
        self.dishes.get(name).cloned()
    }

    pub fn search_dishes(&self, query: &str) -> Vec<Dish> {
        self.dishes
            .iter()
            .filter_map(|(k, v)| {
                if k.contains(query) {
                    Some(v.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn set_last_cooked(&mut self, name: String, time: u64) {
        if let Some(d) = self.dishes.get_mut(&name) {
            d.set_last_cooked(time)
        }
    }
}
