use std::collections::HashMap;

// mod colour;
pub mod errors;
pub mod files;
pub mod colour;

pub fn iter_hashmap<K, V>(map: &mut HashMap<K, V>, func: &dyn Fn(&K, &V)) {
   for (key, value) in &*map {
      func(key, value)
   }
}
