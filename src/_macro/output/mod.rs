#[macro_export]
macro_rules! clone_output {
  ($data: expr) => {{
    #[allow(unused_imports)]
    use std::io::Write as _;
    let output_resource = $data.output_resource.0.as_ref().unwrap();
    output_resource.clone()
  }};
}
