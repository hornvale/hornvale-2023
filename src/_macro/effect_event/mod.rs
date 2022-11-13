#[macro_export]
macro_rules! get_effect_event_channel {
  ($data: expr) => {{
    &mut $data.effect_event_channel
  }};
}

#[macro_export]
macro_rules! write_effect_event {
  ($data: expr, $effect: expr) => {{
    #[allow(unused_imports)]
    use $crate::event::EffectEvent;
    get_effect_event_channel!($data).single_write(EffectEvent { effect: $effect });
  }};
}
