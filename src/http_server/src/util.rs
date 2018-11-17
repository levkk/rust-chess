use std::collections::HashMap;

/// Useful filter with a lambda function, à la JavaScript Array.filter()[0]
pub trait Filterable<K, V> {
  fn get_filter_mut(&mut self, func: &Fn(&V) -> bool) -> Option<&mut V>;
  fn get_filter(&self, func: &Fn(&V) -> bool) -> Option<V>;
}

impl <K: std::cmp::Eq + std::hash::Hash, V: std::clone::Clone>Filterable<K, V> for HashMap<K, V> {
  fn get_filter_mut(&mut self, func: &Fn(&V) -> bool) -> Option<&mut V> {
    for (_key, mut value) in self.iter_mut() {
      if func(&value.clone()) {
        return Some(value);
      }
    }

    None
  }

  fn get_filter(&self, func: &Fn(&V) -> bool) -> Option<V> {
    for (_key, value) in self.iter() {
      if func(&value.clone()) {
        return Some(value.clone());
      }
    }

    None
  }
}
