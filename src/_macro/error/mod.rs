#[macro_export]
macro_rules! error_message {
  ($system_data: expr, $string: expr) => {{
    #[allow(unused_imports)]
    use $crate::ecs::event_channels::OutputEvent;
    get_output!($system_data).single_write(OutputEvent { string: $string });
  }};
}
