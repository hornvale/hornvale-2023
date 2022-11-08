#[macro_export]
macro_rules! get_output {
  ($system_data: expr) => {{
    #[allow(unused_imports)]
    use std::io::Write as _;
    let output_resource = $system_data.output_resource.0.as_ref().unwrap();
    output_resource.clone()
  }};
}
