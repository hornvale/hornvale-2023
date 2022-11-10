#[macro_export]
macro_rules! get_camera {
  ($system_data: expr) => {{
    $system_data.camera_resource.0.unwrap()
  }};
}

#[macro_export]
macro_rules! has_camera {
  ($system_data: expr, $entity_id: expr) => {{
    *$entity_id == get_camera!($system_data)
  }};
}
