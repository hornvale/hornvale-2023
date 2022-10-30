#![allow(unused_imports)]
#![allow(unused_variables)]
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_fly_camera::{FlyCamera2d, FlyCameraPlugin};
use bevy_prototype_lyon::prelude::*;
use derive_more::Deref;
use hornvale::astronomy::star::Star;
use hornvale::astronomy::star_subsystem::StarSubsystem;
use hornvale::astronomy::star_system::constraints::Constraints;
use hornvale::astronomy::star_system::StarSystem;
use std::collections::HashMap;
use std::f32::consts::PI;

#[macro_use]
extern crate function_name;

/* REWRITE
#[derive(Default)]
struct Stats {
  frame_number: usize,
  n_objects: usize,
  draw_traces: bool,
}

const GRAVITATIONAL_CONSTANT: f32 = 5.0;

struct ClearTraces;
struct Reset;

#[derive(Component, Debug, Clone, Deref)]
struct Velocity(Vec2);

#[derive(Component, Debug, Clone)]
struct Planet {
  radius: f32,
  mass: f32,
  color: Color,
  is_sun: bool,
}

#[derive(Component)]
struct Trace {
  live_until: f64,
}

impl Planet {}

fn gravity(
  mut commands: Commands,
  mut planet_query: Query<(Entity, &mut Planet, &mut Velocity, &mut Transform)>,
  mut stats: ResMut<Stats>,
  time: Res<Time>,
) {
  let mut accel_map: HashMap<u32, Vec2> = HashMap::new();
  stats.n_objects = 0;
  let mut largest = 0.0;
  stats.frame_number += 1;

  for (entity_1, planet_1, velocity_1, transform_1) in planet_query.iter() {
    if stats.frame_number % 5 == 0 && stats.draw_traces {
      let mut transform: Transform = *transform_1;
      transform.translation.z = 100.0;
      spawn_trace(&mut commands, transform, time.seconds_since_startup() + 60.0);
    }
    if planet_1.radius > largest && !planet_1.is_sun {
      largest = planet_1.radius;
    }
    stats.n_objects += 1;
    let mut accel_cum = Vec2::new(0.0, 0.0);
    for (entity_2, planet_2, velocity_2, transform_2) in planet_query.iter() {
      if entity_1.id() != entity_2.id() {
        let r_vector = transform_1.translation.truncate() - transform_2.translation.truncate();
        let r_mag = r_vector.length();
        let r_mag = if r_mag < planet_1.radius + planet_2.radius {
          planet_1.radius + planet_2.radius
        } else {
          r_mag
        };
        let accel: f32 = -1.0 * GRAVITATIONAL_CONSTANT * planet_2.mass / r_mag.powf(2.0) * 50.0;
        let r_vector_unit = r_vector / r_mag;
        accel_cum += accel * r_vector_unit;
      }
    }
    accel_map.insert(entity_1.id(), accel_cum);
  }
  let step = 1.0 / 10.0;
  for (entity_1, _, mut velocity_1, mut transform_1) in planet_query.iter_mut() {
    velocity_1.0 += *accel_map.get(&entity_1.id()).unwrap() * step;
    transform_1.translation.x += velocity_1.x * step;
    transform_1.translation.y += velocity_1.y * step;
  }
}

fn get_star_color(star: &Star) -> Color {
  let star_color = star.absolute_rgb;
  let r = star_color.0 as f32;
  let g = star_color.1 as f32;
  let b = star_color.2 as f32;
  Color::rgb(r / 256.0, g as f32 / 256.0, b / 256.0)
}

fn au_to_distance(au: f64) -> f32 {
  au as f32 * 500.0
}

fn despawn_traces(
  mut ev_clear_trace: EventReader<ClearTraces>,
  mut commands: Commands,
  traces: Query<(Entity, &Trace)>,
  time: Res<Time>,
) {
  let mut manual_clear = false;
  for _ in ev_clear_trace.iter() {
    manual_clear = true;
  }
  for (entity, trace) in traces.iter() {
    if trace.live_until < time.seconds_since_startup() || manual_clear {
      commands.entity(entity).despawn();
    }
  }
}

fn setup(mut commands: Commands, mut ev_reset: EventWriter<Reset>) {
  commands
    .spawn_bundle(OrthographicCameraBundle::new_2d())
    .insert(FlyCamera2d::default());
  ev_reset.send(Reset);
}

fn setup_many_orbits(
  planet_query: Query<(Entity, &mut Planet)>,
  mut ev_reset: EventReader<Reset>,
  mut commands: Commands,
) {
  let mut manual_reset = false;
  for _ in ev_reset.iter() {
    manual_reset = true;
  }
  if manual_reset {
    for (ent, _) in planet_query.iter() {
      commands.entity(ent).despawn();
    }

    let mut rng = rand::thread_rng();
    let center = Vec3::new(0.0, 0.0, 10.0);
    let constraints = Constraints::habitable_close_binary();
    if let Ok(star_system) = StarSystem::from_constraints(&mut rng, &constraints) {
      let StarSubsystem = &star_system.StarSubsystem;
      spawn_subsystem(
        &mut commands,
        &StarSubsystem,
        Velocity(Vec2::new(0.0, 0.0)),
        Transform::from_xyz(center.x, center.y, center.z),
      );
    }
  }
}

fn spawn_subsystem(commands: &mut Commands, star_subsystem: &StarSubsystem, velocity: Velocity, transform: Transform) {
  use StarSubsystemType::*;
  print_var!("{:#?}", star_subsystem);
  match &star_subsystem.r#type {
    Single(star) => spawn_star(commands, &star, velocity, transform),
    Double(binary) => {
      let sub1 = &binary.primary;
      let sub2 = &binary.secondary;
      let distance1 = au_to_distance(binary.average_distances_from_barycenter.0);
      let distance2 = au_to_distance(binary.average_distances_from_barycenter.1);
      let orbital_velocity1 = (GRAVITATIONAL_CONSTANT * sub1.mass as f32 / distance1).sqrt() as f32;
      let orbital_velocity2 = (GRAVITATIONAL_CONSTANT * sub2.mass as f32 / distance2).sqrt() as f32;
      let radian1: f32 = PI;
      let radian2: f32 = 0.0;
      let x1: f32 = distance1 * radian1.cos();
      let y1: f32 = distance1 * radian1.sin();
      let x2: f32 = distance2 * radian2.cos();
      let y2: f32 = distance2 * radian2.sin();
      let vx1: f32 = -orbital_velocity1 * radian1.sin();
      let vy1: f32 = orbital_velocity1 * radian1.cos();
      let vx2: f32 = -orbital_velocity2 * radian2.sin();
      let vy2: f32 = orbital_velocity2 * radian2.cos();
      let vec1 = Vec2::new(vx1, vy1);
      let vec2 = Vec2::new(vx2, vy2);
      let transform1 = Transform::from_xyz(x1, y1, 10.0);
      let transform2 = Transform::from_xyz(x2, y2, 10.0);
      spawn_subsystem(commands, sub1, Velocity(vec1), transform1);
      spawn_subsystem(commands, sub2, Velocity(vec2), transform2);
    },
  }
}

fn spawn_star(commands: &mut Commands, star: &Star, velocity: Velocity, transform: Transform) {
  let planet = Planet {
    radius: star.radius as f32,
    mass: star.mass as f32,
    color: get_star_color(&star),
    is_sun: false,
  };
  spawn_planet(commands, planet.clone(), velocity, transform);
}

fn spawn_planet(commands: &mut Commands, planet: Planet, velocity: Velocity, transform: Transform) {
  let shape = shapes::Circle {
    radius: planet.radius * 10.0,
    center: Default::default(),
  };
  let mut entity_commands = commands.spawn_bundle(GeometryBuilder::build_as(
    &shape,
    DrawMode::Outlined {
      fill_mode: FillMode::color(planet.color),
      outline_mode: StrokeMode::new(planet.color, 0.0),
    },
    transform,
  ));
  entity_commands.insert(planet).insert(velocity);
}

fn spawn_trace(commands: &mut Commands, transform: Transform, live_until: f64) {
  commands
    .spawn_bundle(SpriteBundle {
      sprite: Sprite {
        color: Color::GRAY,
        custom_size: Some(Vec2::new(1.0, 1.0)),
        ..Default::default()
      },
      transform,
      ..Default::default()
    })
    .insert(Trace { live_until });
}

fn ui_box(
  mut ev_clear_traces: EventWriter<ClearTraces>,
  mut ev_reset: EventWriter<Reset>,
  diagnostics: Res<Diagnostics>,
  mut egui_context: ResMut<EguiContext>,
  mut stats: ResMut<Stats>,
) {
  egui::Window::new("Star Visualizer").show(egui_context.ctx_mut(), |ui| {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
      if let Some(average) = fps.average() {
        ui.label("WASD to move");
        ui.label(format!("FPS {:.2}", average));
        ui.label(format!("Number of objects {:}", stats.n_objects));
        ui.checkbox(&mut stats.draw_traces, "Draw traces");
        if ui.button("Clear traces").clicked() {
          ev_clear_traces.send(ClearTraces);
        };
        if ui.button("Reset").clicked() {
          ev_reset.send(Reset);
        }
      }
    }
  });
}

pub fn game() {
  let mut stats = Stats::default();
  stats.draw_traces = true;
  App::new()
    .insert_resource(Msaa { samples: 4 })
    .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
    .add_event::<ClearTraces>()
    .add_event::<Reset>()
    .add_plugins(DefaultPlugins)
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_plugin(EguiPlugin)
    .add_plugin(ShapePlugin)
    .add_plugin(FlyCameraPlugin)
    .add_startup_system(setup)
    .add_system(gravity)
    .add_system(ui_box)
    .add_system(despawn_traces)
    .add_system(setup_many_orbits)
    .insert_resource(stats)
    .run();
}
*/

pub fn main() {
  //  game()
}
