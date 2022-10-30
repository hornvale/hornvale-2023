/// Humanize the size of an object, based on its radius.
///
/// Good for things like balls, planets, etc.
///
/// # Arguments
/// * `radius` - Object radius
///
#[named]
pub fn humanize_sphere_by_radius(radius: f64) -> &'static str {
  trace_enter!();
  let result = match radius {
    radius if radius <= 1.08E21 => "smaller than Earth",
    radius if radius <= 1.08E48 => "larger than Earth",
    _ => "LOL",
  };
  trace_var!(result);
  trace_exit!();
  result
}
