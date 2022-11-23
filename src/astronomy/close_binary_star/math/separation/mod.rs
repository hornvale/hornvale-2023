use crate::astronomy::_type::*;

pub fn get_minimum_separation(min_distances: (LAu, LAu)) -> LAu {
  min_distances.0 + min_distances.1
}

pub fn get_maximum_separation(max_distances: (LAu, LAu)) -> LAu {
  max_distances.0 + max_distances.1
}
